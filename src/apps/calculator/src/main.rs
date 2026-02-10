use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Settings, Shadow, Theme, Vector};
use iced::event::{self, Event};
use iced::keyboard::{self, Key};
use iced::keyboard::key::Named;
use iced::{Application, Command, Subscription};

pub fn main() -> iced::Result {
    Calculadora::run(Settings::default())
}

#[derive(Default)]
struct Calculadora {
    display: String,
    operando1: Option<f64>,
    operador: Option<Operador>,
    nueva_entrada: bool,
}

#[derive(Debug, Clone, Copy)]
enum Operador {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Potencia,
    Porcentaje,
}

#[derive(Debug, Clone)]
enum Mensaje {
    NumeroPresionado(char),
    OperadorPresionado(Operador),
    Igual,
    Limpiar,
    Borrar,
    Punto,
    RaizCuadrada,
    EventoOcurrido(Event),
}

impl Application for Calculadora {
    type Executor = iced::executor::Default;
    type Message = Mensaje;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Mensaje>) {
        (
            Self {
                display: String::from("0"),
                operando1: None,
                operador: None,
                nueva_entrada: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculadora Minimalista")
    }

    fn subscription(&self) -> Subscription<Mensaje> {
        event::listen().map(Mensaje::EventoOcurrido)
    }

    fn update(&mut self, mensaje: Mensaje) -> Command<Mensaje> {
        match mensaje {
            Mensaje::EventoOcurrido(evento) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = evento {
                    match key.as_ref() {
                        Key::Character(c) => {
                            let ch = c.chars().next().unwrap_or('\0');
                            match ch {
                                '0'..='9' => self.procesar_numero(ch),
                                '+' => self.procesar_operador(Operador::Suma),
                                '-' => self.procesar_operador(Operador::Resta),
                                '*' | 'x' | 'X' => self.procesar_operador(Operador::Multiplicacion),
                                '/' => self.procesar_operador(Operador::Division),
                                '^' => self.procesar_operador(Operador::Potencia),
                                '%' => self.procesar_operador(Operador::Porcentaje),
                                '.' | ',' => self.procesar_punto(),
                                'c' | 'C' => self.limpiar(),
                                'r' | 'R' => self.raiz_cuadrada(),
                                _ => {}
                            }
                        }
                        Key::Named(Named::Enter) => self.calcular_igual(),
                        Key::Named(Named::Backspace) => self.borrar(),
                        Key::Named(Named::Escape) => self.limpiar(),
                        _ => {}
                    }
                }
            }
            Mensaje::NumeroPresionado(num) => self.procesar_numero(num),
            Mensaje::OperadorPresionado(op) => self.procesar_operador(op),
            Mensaje::Igual => self.calcular_igual(),
            Mensaje::Limpiar => self.limpiar(),
            Mensaje::Borrar => self.borrar(),
            Mensaje::Punto => self.procesar_punto(),
            Mensaje::RaizCuadrada => self.raiz_cuadrada(),
        }
        Command::none()
    }

    fn view(&self) -> Element<Mensaje> {
        let display = container(
            text(&self.display)
                .size(56)
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Right),
        )
            .width(Length::Fill)
            .padding(25)
            .style(estilo_display);

        let fila0 = row![
            boton_funcion("C", Mensaje::Limpiar),
            boton_funcion("⌫", Mensaje::Borrar),
            boton_funcion("√", Mensaje::RaizCuadrada),
            boton_operador("^", Operador::Potencia),
        ]
            .spacing(12);

        let fila1 = row![
            boton_numero('7'),
            boton_numero('8'),
            boton_numero('9'),
            boton_operador("÷", Operador::Division),
        ]
            .spacing(12);

        let fila2 = row![
            boton_numero('4'),
            boton_numero('5'),
            boton_numero('6'),
            boton_operador("×", Operador::Multiplicacion),
        ]
            .spacing(12);

        let fila3 = row![
            boton_numero('1'),
            boton_numero('2'),
            boton_numero('3'),
            boton_operador("-", Operador::Resta),
        ]
            .spacing(12);

        let fila4 = row![
            boton_especial("0"),
            boton_funcion(".", Mensaje::Punto),
            boton_operador("%", Operador::Porcentaje),
            boton_operador("+", Operador::Suma),
        ]
            .spacing(12);

        let fila5 = row![boton_igual("=")].spacing(12);

        let contenido = column![display, fila0, fila1, fila2, fila3, fila4, fila5]
            .spacing(14)
            .align_items(Alignment::Center)
            .padding(20);

        container(contenido)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .style(estilo_fondo)
            .into()
    }
}

impl Calculadora {
    fn procesar_numero(&mut self, num: char) {
        if self.nueva_entrada || self.display == "0" {
            self.display = num.to_string();
            self.nueva_entrada = false;
        } else if self.display.len() < 15 {
            self.display.push(num);
        }
    }

    fn procesar_punto(&mut self) {
        if self.nueva_entrada {
            self.display = String::from("0.");
            self.nueva_entrada = false;
        } else if !self.display.contains('.') && self.display.len() < 15 {
            self.display.push('.');
        }
    }

    fn procesar_operador(&mut self, op: Operador) {
        if let Ok(valor) = self.display.parse::<f64>() {
            if let (Some(op1), Some(operador_previo)) = (self.operando1, self.operador) {
                let resultado = calcular(op1, valor, operador_previo);
                self.display = formatear_resultado(resultado);
                if self.display == "Error" {
                    self.operando1 = None;
                    self.operador = None;
                    self.nueva_entrada = true;
                    return;
                }
                self.operando1 = resultado;
            } else {
                self.operando1 = Some(valor);
            }
            self.operador = Some(op);
            self.nueva_entrada = true;
        }
    }

    fn calcular_igual(&mut self) {
        if let (Some(op1), Some(op), Ok(op2)) = (self.operando1, self.operador, self.display.parse::<f64>()) {
            let resultado = calcular(op1, op2, op);
            self.display = formatear_resultado(resultado);
            if self.display == "Error" {
                self.operando1 = None;
                self.operador = None;
                self.nueva_entrada = true;
                return;
            }
            self.operando1 = None;
            self.operador = None;
            self.nueva_entrada = true;
        }
    }

    fn limpiar(&mut self) {
        self.display = String::from("0");
        self.operando1 = None;
        self.operador = None;
        self.nueva_entrada = true;
    }

    fn borrar(&mut self) {
        if !self.nueva_entrada && self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = String::from("0");
            self.nueva_entrada = true;
        }
    }

    fn raiz_cuadrada(&mut self) {
        if let Ok(valor) = self.display.parse::<f64>() {
            if valor >= 0.0 {
                let resultado = Some(valor.sqrt());
                self.display = formatear_resultado(resultado);
                if self.display == "Error" {
                    self.nueva_entrada = true;
                    return;
                }
                self.nueva_entrada = true;
            } else {
                self.display = String::from("Error");
                self.nueva_entrada = true;
            }
        }
    }
}

// ========== ESTILOS PERSONALIZADOS ==========

fn estilo_fondo(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.95, 0.95, 0.97).into()),
        ..Default::default()
    }
}

fn estilo_display(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.13, 0.13, 0.16).into()),
        text_color: Some(Color::WHITE),
        border: Border {
            radius: 20.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

// En Iced 0.12+, la API cambió. Ahora usamos un enum para el estado del botón
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EstadoBoton {
    Activo,
    Sobre,
    Presionado,
    Deshabilitado,
}

fn estilo_boton_numero(estado: EstadoBoton) -> button::Appearance {
    let base = button::Appearance {
        background: Some(Color::WHITE.into()),
        text_color: Color::from_rgb(0.1, 0.1, 0.12),
        border: Border {
            radius: 16.0.into(),
            width: 1.5,
            color: Color::from_rgb(0.88, 0.88, 0.90),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
        shadow_offset: Vector::new(0.0, 0.0),
    };

    match estado {
        EstadoBoton::Sobre => button::Appearance {
            background: Some(Color::from_rgb(0.98, 0.98, 0.99).into()),
            border: Border {
                radius: 16.0.into(),
                width: 2.0,
                color: Color::from_rgb(0.4, 0.6, 1.0),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            shadow_offset: Vector::new(0.0, 2.0),
            ..base
        },
        EstadoBoton::Presionado => button::Appearance {
            background: Some(Color::from_rgb(0.92, 0.92, 0.95).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
            ..base
        },
        _ => base,
    }
}

fn estilo_boton_operador(estado: EstadoBoton) -> button::Appearance {
    let base = button::Appearance {
        background: Some(Color::from_rgb(0.98, 0.6, 0.2).into()),
        text_color: Color::WHITE,
        border: Border {
            radius: 16.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 6.0,
        },
        shadow_offset: Vector::new(0.0, 0.0),
    };

    match estado {
        EstadoBoton::Sobre => button::Appearance {
            background: Some(Color::from_rgb(1.0, 0.7, 0.3).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 5.0),
                blur_radius: 10.0,
            },
            shadow_offset: Vector::new(0.0, 2.0),
            ..base
        },
        EstadoBoton::Presionado => button::Appearance {
            background: Some(Color::from_rgb(0.9, 0.55, 0.15).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
            ..base
        },
        _ => base,
    }
}

fn estilo_boton_funcion(estado: EstadoBoton) -> button::Appearance {
    let base = button::Appearance {
        background: Some(Color::from_rgb(0.88, 0.88, 0.90).into()),
        text_color: Color::from_rgb(0.2, 0.2, 0.24),
        border: Border {
            radius: 16.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 6.0,
        },
        shadow_offset: Vector::new(0.0, 0.0),
    };

    match estado {
        EstadoBoton::Sobre => button::Appearance {
            background: Some(Color::from_rgb(0.92, 0.92, 0.94).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
            ..base
        },
        EstadoBoton::Presionado => button::Appearance {
            background: Some(Color::from_rgb(0.8, 0.8, 0.84).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
            ..base
        },
        _ => base,
    }
}

fn estilo_boton_igual(estado: EstadoBoton) -> button::Appearance {
    let base = button::Appearance {
        background: Some(Color::from_rgb(0.2, 0.72, 0.42).into()),
        text_color: Color::WHITE,
        border: Border {
            radius: 16.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        },
        shadow_offset: Vector::new(0.0, 0.0),
    };

    match estado {
        EstadoBoton::Sobre => button::Appearance {
            background: Some(Color::from_rgb(0.25, 0.8, 0.5).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: Vector::new(0.0, 6.0),
                blur_radius: 12.0,
            },
            shadow_offset: Vector::new(0.0, 2.0),
            ..base
        },
        EstadoBoton::Presionado => button::Appearance {
            background: Some(Color::from_rgb(0.15, 0.65, 0.35).into()),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
            ..base
        },
        _ => base,
    }
}

// Crear implementaciones de StyleSheet para cada tipo de botón
#[derive(Debug, Clone, Copy)]
struct EstiloBotonNumero;

impl button::StyleSheet for EstiloBotonNumero {
    type Style = Theme;

    // Prefix unused variables with _style to silence warnings
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_numero(EstadoBoton::Activo)
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_numero(EstadoBoton::Sobre)
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_numero(EstadoBoton::Presionado)
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_numero(EstadoBoton::Deshabilitado)
    }
}

#[derive(Debug, Clone, Copy)]
struct EstiloBotonOperador;

impl button::StyleSheet for EstiloBotonOperador {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_operador(EstadoBoton::Activo)
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_operador(EstadoBoton::Sobre)
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_operador(EstadoBoton::Presionado)
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_operador(EstadoBoton::Deshabilitado)
    }
}

#[derive(Debug, Clone, Copy)]
struct EstiloBotonFuncion;

impl button::StyleSheet for EstiloBotonFuncion {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_funcion(EstadoBoton::Activo)
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_funcion(EstadoBoton::Sobre)
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_funcion(EstadoBoton::Presionado)
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_funcion(EstadoBoton::Deshabilitado)
    }
}

#[derive(Debug, Clone, Copy)]
struct EstiloBotonIgual;

impl button::StyleSheet for EstiloBotonIgual {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_igual(EstadoBoton::Activo)
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_igual(EstadoBoton::Sobre)
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_igual(EstadoBoton::Presionado)
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        estilo_boton_igual(EstadoBoton::Deshabilitado)
    }
}

// ========== FUNCIONES DE BOTONES ==========

fn boton_numero(num: char) -> Element<'static, Mensaje> {
    button(
        text(num)
            .size(28)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
        .on_press(Mensaje::NumeroPresionado(num))
        .width(75)
        .height(68)
        .style(iced::theme::Button::Custom(Box::new(EstiloBotonNumero)))
        .into()
}

fn boton_especial(label: &'static str) -> Element<'static, Mensaje> {
    button(
        text(label)
            .size(28)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
        .on_press(Mensaje::NumeroPresionado('0'))
        .width(165)
        .height(68)
        .style(iced::theme::Button::Custom(Box::new(EstiloBotonNumero)))
        .into()
}

fn boton_operador(simbolo: &'static str, op: Operador) -> Element<'static, Mensaje> {
    button(
        text(simbolo)
            .size(28)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
        .on_press(Mensaje::OperadorPresionado(op))
        .width(75)
        .height(68)
        .style(iced::theme::Button::Custom(Box::new(EstiloBotonOperador)))
        .into()
}

fn boton_funcion(simbolo: &'static str, mensaje: Mensaje) -> Element<'static, Mensaje> {
    button(
        text(simbolo)
            .size(26)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
        .on_press(mensaje)
        .width(75)
        .height(68)
        .style(iced::theme::Button::Custom(Box::new(EstiloBotonFuncion)))
        .into()
}

fn boton_igual(simbolo: &'static str) -> Element<'static, Mensaje> {
    button(
        text(simbolo)
            .size(32)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
        .on_press(Mensaje::Igual)
        .width(337)
        .height(68)
        .style(iced::theme::Button::Custom(Box::new(EstiloBotonIgual)))
        .into()
}

// ========== LÓGICA DE CÁLCULO ==========

fn calcular(a: f64, b: f64, op: Operador) -> Option<f64> {
    match op {
        Operador::Suma => Some(a + b),
        Operador::Resta => Some(a - b),
        Operador::Multiplicacion => Some(a * b),
        Operador::Division => {
            if b != 0.0 {
                Some(a / b)
            } else {
                None // División por cero
            }
        }
        Operador::Potencia => {
            let res = a.powf(b);
            if res.is_finite() { Some(res) } else { None }
        }
        Operador::Porcentaje => Some(a * (b / 100.0)),
    }
}

fn formatear_resultado(num: Option<f64>) -> String {
    match num {
        Some(n) if n.is_finite() => {
            if n.fract() == 0.0 && n.abs() < 1e10 {
                format!("{}", n as i64)
            } else {
                let resultado = format!("{:.10}", n);
                resultado.trim_end_matches('0').trim_end_matches('.').to_string()
            }
        }
        _ => "Error".to_string(),
    }
}