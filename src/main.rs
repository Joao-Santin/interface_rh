use std::{fs, fmt, path::PathBuf};
use rfd::FileDialog;
use encoding_rs::WINDOWS_1252; // ou ISO_8859_1, se preferir
use iced::{Color, Element, Task as Command};
use iced::widget::{button, column, container, row, scrollable, text, Column, Space, Text};
use iced::{Alignment::{Center}, Length::{self, Fill, Fixed}};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
//
#[derive(Debug, Clone)]
enum Screens{
    Main,
    Calendar
    // LobbyColab,
}
// enum DecodeTypes{
//     WinUTF
// }
struct AFDBase{
    nsr: String,
    tipo: RegistryTypes
    
}
struct Cabecalho{
    base: AFDBase,
    tipo_empregador: String,
    cnpj_empregador: String,
    cno_empregador: String,
    razao_social: String,
    id_rep: String,
    data_inicio: String,
    data_final: String,
    geracao_arquivo: String,
    versao_leiaute_afd: String,
    cnpj_fabricante_rep: String,
    modelo_rep: String,
    registro_hexa: String,
}

struct CreateUpdateEmpresa{
    base: AFDBase,
    date_time: String,
    cpf_admin: String,
    tipo_empregador: String,
    cnpj_empregador: String,
    cno: String,
    razao_social: String,
    local_servico: String,
    registro_hexa: String,
}
struct MarcacaoPonto{
    base: AFDBase,
    date_time: String,
    cpf_empregado: String,
    registro_hexa: String,
}
struct AjusteRelogio{
    base: AFDBase,
    date_time_antes_registro: String,
    date_time_ajustado: String,
    cpf_adm: String,
    registro_hexa: String,

}
struct CreateaUpdateDeleteEmpregado{
    base: AFDBase,
    date_time: String,
    tipo_operacao: String,
    cpf_empregado: String,
    nome_empregado: String,
    mais_dados_empregado: String,
    cpf_adm: String,
    registro_hexa: String,
}

struct SensivelREP{
    base: AFDBase,
    date_time: String,
    evento: String,
}
// struct MarcacaoPontoRepP{
//     base: AFDBase
//
// }
// struct Trailer{
//     base: AFDBase
//
// }

enum RegistryTypes{
    Cabecalho,
    CreateUpdateEmpresa,
    MarcacaoPonto,
    AjusteRelogio,
    CreateUpdateDeleteEmpregado,
    SensivelREP,
    MarcacaoPontoRepP,
    Trailer,
}

impl RegistryTypes{
    fn get_type_by_number(number: i8)->Option<Self>{
        match number{
            1 => Some(Self::Cabecalho),
            2 => Some(Self::CreateUpdateEmpresa),
            3 => Some(Self::MarcacaoPonto),
            4 => Some(Self::AjusteRelogio),
            5 => Some(Self::CreateUpdateDeleteEmpregado),
            6 => Some(Self::SensivelREP),
            7 => Some(Self::MarcacaoPontoRepP),
            9 => Some(Self::Trailer),
            _ => None,
        }
    }

    fn parse(&self, interfacerh: &mut InterfaceRH, linha: &str){
        match self{
            Self::Cabecalho => {
                let n_serie = &linha[0..9];
                let tipo = RegistryTypes::Cabecalho;
                println!("{}", tipo);
                let mut tipo_empregador_txt = String::new();
                if let Some(char_casa_11) = linha.chars().nth(10){
                    let tipo_empregador = char_casa_11.to_digit(10).unwrap() as i8;
                    if tipo_empregador == 1{
                        tipo_empregador_txt = "CNPJ".to_string()
                    }else{
                        tipo_empregador_txt = "CPF".to_string()
                    };
                    // println!("tipo_empregador: {}", tipo_empregador_txt)

                }
                let cnpj_cpf = &linha[11..25];
                // println!("cnpj/cpj: {}", cnpj_cpf);
                let cno_caepf = &linha[25..39];
                // println!("cno: {}", cno_caepf);
                let razao_social = &linha[39..189];
                // println!("razao_social: {}", razao_social.to_string().trim());
                let id_rep = &linha[189..206];
                // println!("id_rep: {}", id_rep);

                let data_inicio = &linha[206..216];
                // println!("data_inicio: {}", data_inicio);

                let data_final = &linha[216..226];
                // println!("data_final: {}", data_final);

                let geracao_arquivo = &linha[226..250];
                // println!("data/hora geracao arquivo: {}", geracao_arquivo);

                let versao_leiaute_afd = &linha[250..253];
                // println!("leiaute_afd: {}", versao_leiaute_afd);

                let tipo_fabricante_rep_char = &linha[253..254];
                let tipo_fabricante_rep = if tipo_fabricante_rep_char.to_string() == "1"{"CNPJ"}else{"CPF"};
                // println!("Tipo fabricante: {}", tipo_fabricante_rep);

                let cnpj_fabricante_rep = &linha[254..268];
                // println!("CNPJ fabricante rep: {}", cnpj_fabricante_rep.to_string().trim());

                let modelo_rep = &linha[268..298];
                // println!("Modelo REP: {}", modelo_rep.to_string().trim());

                let registro_hexa = &linha[298..302];
                // println!("Registro hexa: {}", registro_hexa);

                let cabecalho = Cabecalho{
                    base: AFDBase { nsr: n_serie.to_string(), tipo: tipo },
                    tipo_empregador: tipo_empregador_txt,
                    cnpj_empregador: cnpj_cpf.to_string(),
                    cno_empregador: cno_caepf.to_string(),
                    razao_social: razao_social.to_string(),
                    id_rep: id_rep.to_string(),
                    data_inicio: data_inicio.to_string(),
                    data_final: data_final.to_string(),
                    geracao_arquivo: geracao_arquivo.to_string(),
                    versao_leiaute_afd: versao_leiaute_afd.to_string(),
                    cnpj_fabricante_rep: cnpj_fabricante_rep.to_string(),
                    modelo_rep: modelo_rep.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                interfacerh.data.cabecalho = Some(cabecalho)
            },

            Self::CreateUpdateEmpresa => { //2
                let nsr = &linha[0..9];
                let registro = RegistryTypes::CreateUpdateEmpresa;
                let date_time = &linha[10..34];
                let cpf_admin = &linha[34..48];
                let tipo_empregador_txt = &linha[48..49];
                let tipo_empregador_int: i8 = tipo_empregador_txt.parse::<i8>().unwrap();
                let tipo_empregador = match tipo_empregador_int{
                    1 => "CNPJ",
                    2 => "CPF",
                    _ => "NAO LISTADO",

                };
                let cnpj_empregador = &linha[49..63];
                let cno = &linha[63..77];
                let razao_social = &linha[77..227];
                let local_servico = &linha[227..327];
                let registro_hexa = &linha[327..332];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("date time: {}", date_time);
                // println!("cpf_admin: {}", cpf_admin);
                // println!("tipo empregador: {}", tipo_empregador);
                // println!("cnpj empregador: {}", cnpj_empregador.trim());
                // println!("cno: {}", cno);
                // println!("razao_social: {}", razao_social);
                // println!("local_servico: {}", local_servico);
                // println!("registro_hexa: {}", registro_hexa);
                let createupdateempresa = CreateUpdateEmpresa{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: date_time.to_string(),
                    cpf_admin: cpf_admin.to_string(),
                    tipo_empregador: tipo_empregador.to_string(),
                    cnpj_empregador: cnpj_empregador.to_string(),
                    cno: cno.to_string(),
                    razao_social: razao_social.to_string(),
                    local_servico: local_servico.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                interfacerh.data.createupdateempresa.push(createupdateempresa)
            },
            
            Self::MarcacaoPonto => { //3
                let nsr = &linha[0..9];
                let registro = RegistryTypes::MarcacaoPonto;
                let date_time = &linha[10..34];
                let cpf_empregado = &linha[35..46];
                let registro_hexa = &linha[46..50];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("cpf empregado: {}", cpf_empregado);
                // println!("registro hexa: {}", registro_hexa);
                let marcacaoponto = MarcacaoPonto{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: date_time.to_string(),
                    cpf_empregado: cpf_empregado.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                interfacerh.data.marcacaoponto.push(marcacaoponto)
            },

            Self::AjusteRelogio => { //4
                let nsr = &linha[0..9];
                let registro = RegistryTypes::AjusteRelogio;
                let date_time_antes_registro = &linha[10..34];
                let date_time_ajustado = &linha[34..58];
                let cpf_adm = &linha[58..69];
                let registro_hexa = &linha[69..73];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("date_time_antes_registro: {}", date_time_antes_registro);
                // println!("date time ajustado: {}", date_time_ajustado);
                // println!("cpf adm: {}", cpf_adm);
                // println!("registro_hexa: {}", registro_hexa);
                let ajuste_relogio = AjusteRelogio{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time_antes_registro: date_time_antes_registro.to_string(),
                    date_time_ajustado: date_time_ajustado.to_string(),
                    cpf_adm: cpf_adm.to_string(),
                    registro_hexa: registro_hexa.to_string()
                };
                interfacerh.data.ajusterelogio.push(ajuste_relogio)
            },
            Self::CreateUpdateDeleteEmpregado => { //5
                let nsr = &linha[0..9];
                let registro = RegistryTypes::CreateUpdateDeleteEmpregado;
                let date_time = &linha[10..34];
                let tipo_operacao_str = &linha[34..35];
                let tipo_operacao = match tipo_operacao_str{
                    "I" => "Inclusao",
                    "A" => "Alteracao",
                    "E" => "Exclusao",
                    _ => "Evento nao listado"
                };
                let cpf_empregado = &linha[36..47];
                let nome_empregado = &linha[47..99];
                let mais_dados_empregado = &linha[100..104];
                let cpf_adm = &linha[104..115];
                let registro_hexa = &linha[115..118];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("tipo operacao: {}", tipo_operacao);
                // println!("cpf empregado: {}", cpf_empregado);
                // println!("nome empregado: {}", nome_empregado.trim());
                // println!("mais_dados_empregado: {}", mais_dados_empregado);
                // println!("cpf admin: {}", cpf_adm);
                // println!("registro hexa: {}", registro_hexa);
                let createupdatedeleteempregado = CreateaUpdateDeleteEmpregado{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: date_time.to_string(),
                    tipo_operacao: tipo_operacao.to_string(),
                    cpf_empregado: cpf_empregado.to_string(),
                    nome_empregado: nome_empregado.to_string(),
                    mais_dados_empregado: mais_dados_empregado.to_string(),
                    cpf_adm: cpf_adm.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                interfacerh.data.createupdatedeleteempregado.push(createupdatedeleteempregado)

            },

            Self::SensivelREP=> { //6
                let nsr = &linha[0..9];
                let registro = RegistryTypes::SensivelREP;
                let date_time = &linha[10..34];
                let tipo_evento_str = &linha[34..36];
                let tipo_evento_int: i8 = tipo_evento_str.parse::<i8>().unwrap();
                let evento = match tipo_evento_int{
                    1 => "Abertura/Violacao REP",
                    2 => "Retorno Energia",
                    3 => "Introducao de Pendrive",
                    4 => "Retirada de Pendrive",
                    5 => "Emissao da Relacao de Marcacoes",
                    6 => "Erro de impressão",
                    7 => "Disponibilidade de Servico",
                    8 => "Indisponibilidade de Servico",
                    _ => "Evento nao listado"
                };
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("tipo evento: {} -> {}", tipo_evento_str, evento);

                let sensivelrep = SensivelREP{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: date_time.to_string(),
                    evento: evento.to_string()
                };
                interfacerh.data.sensivelrep.push(sensivelrep)
            },
            
            Self::MarcacaoPontoRepP => println!("TODO"),//7
            Self::Trailer => println!("TODO"),//9
        }
    }
}
impl fmt::Display for RegistryTypes{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let s= match self{
            Self::Cabecalho => "Cabecalho",
            Self::CreateUpdateEmpresa => "CreateUpdateEmpresa",
            Self::MarcacaoPonto => "MarcacaoPonto",
            Self::AjusteRelogio => "AjusteRelogio",
            Self::CreateUpdateDeleteEmpregado=> "CreateUpdateDeleteEmpregado",
            Self::SensivelREP => "SensivelREP",
            Self::MarcacaoPontoRepP => "MarcacaoPontoRepP",
            Self::Trailer => "Trailer", 
        };
        write!(f, "{}", s)
    }
}
struct SelDate{
    weekday: chrono::Weekday,
    day: u8,
    month: u8,
    year: u32,
}
impl Default for SelDate{
    fn default()->Self{
        let agora_local = Local::now();
        let dia = agora_local.day();
        let mes = agora_local.month();
        let ano = agora_local.year();
        let weekday = NaiveDate::from_ymd(ano, mes, dia).weekday(); 

        Self{
            weekday, 
            day: dia as u8,
            month: mes as u8,
            year: ano as u32
        }
    }
}
impl SelDate{
    fn get_week_day(&self)->chrono::Weekday{
        let month = self.month as u32;
        let date = NaiveDate::from_ymd(self.year as i32, month, self.day as u32);
        date.weekday()
    }
    fn get_month_day_weekday(&self)->Vec<(chrono::Weekday, u32)>{
        let month = self.month as u32;
        let mut date = NaiveDate::from_ymd_opt(self.year as i32, month, 1).expect("data invalida");
        let mut days = Vec::new();
        while date.month() == month{
            days.push((date.weekday(), date.day()));
            date += Duration::days(1);
        }
        days
    }
}

struct InterfaceRHData{
    cabecalho: Option<Cabecalho>,
    createupdateempresa: Vec<CreateUpdateEmpresa>,
    marcacaoponto: Vec<MarcacaoPonto>,
    ajusterelogio: Vec<AjusteRelogio>,
    createupdatedeleteempregado: Vec<CreateaUpdateDeleteEmpregado>,
    sensivelrep: Vec<SensivelREP>,
    // marcacaopontorepp
    // trailer
}
impl Default for InterfaceRHData{
    fn default() -> Self{
        Self{
            cabecalho: None,
            createupdateempresa: Vec::new(),
            marcacaoponto: Vec::new(),
            ajusterelogio: Vec::new(),
            createupdatedeleteempregado: Vec::new(),
            sensivelrep: Vec::new()
        }
    }
}

struct InterfaceRH{
    screen: Screens,
    last_afd_got: Option<DateTime<Local>>,
    sel_date: SelDate,
    data: InterfaceRHData
}
#[derive(Debug, Clone)]
enum UpDownValue{
    Year,
    Month,
}

#[derive(Debug, Clone)]
enum Buttons{
    GetAFDFile,
    SwitchTo(Screens),
    UpDownButton(i32, UpDownValue),
    SelDay(u32)
}

#[derive(Debug, Clone)]
enum Message{
    ButtonPressed(Buttons)
}

impl Default for InterfaceRH{
    fn default() -> Self{
        Self{
            screen: Screens::Main,
            last_afd_got: None,
            sel_date: SelDate::default(),
            data: InterfaceRHData::default()
        }
    }
}

impl InterfaceRH{
    fn int_to_month_pt(mes_int: u8) -> Option<&'static str>{
        match mes_int{
            1=>Some("Janeiro"),
            2=>Some("Fevereiro"),
            3=>Some("Marco"),
            4=>Some("Abril"),
            5=>Some("Maio"),
            6=>Some("Junho"),
            7=>Some("Julho"),
            8=>Some("Agosto"),
            9=>Some("Setembro"),
            10=>Some("Outubro"),
            11=>Some("Novembro"),
            12=>Some("Dezembro"),
            _=>None
        }
    }
    fn weekday_pt(wd: chrono::Weekday) -> &'static str{
        match wd{
            chrono::Weekday::Mon => "Segunda-feira",
            chrono::Weekday::Tue => "Terca-feira",
            chrono::Weekday::Wed => "Quarta-feira",
            chrono::Weekday::Thu => "Quinta-feira",
            chrono::Weekday::Fri => "Sexta-feira",
            chrono::Weekday::Sat => "Sabado",
            chrono::Weekday::Sun => "Domingo"
        }
    }
    fn decode_from_win1252_to_utf8(&mut self, path: PathBuf){
        match fs::read(&path) {
            Ok(bytes) => {
                // Tenta decodificar de Windows-1252 para UTF-8
                let (conteudo, _, _) = WINDOWS_1252.decode(&bytes);

                // Aqui você pode começar a fazer o parse:
                self.data = InterfaceRHData::default();
                
                for linha in conteudo.lines() {
                    InterfaceRH::parse_rep_line(self, linha)
                };
                
            }
            Err(e) => eprintln!("Erro ao ler o arquivo: {}", e),
        }

    }
    fn parse_rep_line(&mut self, linha: &str){
        if let Some(c) = linha.chars().nth(9){
            let n: i8 = c.to_digit(10).unwrap_or(0) as i8;
            if let Some(tipo_registry) = RegistryTypes::get_type_by_number(n){
                tipo_registry.parse(self, linha)
            }else{
                println!("nao tem tipo com esse numero...")
            }

        }else{
            println!("Sem caractere na posicao 10")
        }
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message{
            Message::ButtonPressed(button) => {
                // println!("A Button got pressed!");
                match button{
                    Buttons::GetAFDFile => {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Arquivos de texto", &["txt"])
                            .set_title("SELECIONE O ARQUIVO DE PONTO!")
                            .pick_file()
                        {
                            InterfaceRH::decode_from_win1252_to_utf8(self, path);
                            let agora_local: DateTime<Local> = Local::now();
                            self.last_afd_got = Some(agora_local);

                        } else {
                            println!("Nenhum arquivo selecionado.");
                        }
                    },
                    Buttons::SwitchTo(screen) => {
                        // println!("Mudando de tela!");
                        match screen{
                            Screens::Main => self.screen = Screens::Main,
                            Screens::Calendar => {
                                self.screen = Screens::Calendar
                            }
                        }
                    }
                    Buttons::UpDownButton(delta, campo) => {
                        match campo{
                            UpDownValue::Year => {
                                if delta >= 0 {
                                    self.sel_date.year = self.sel_date.year.saturating_add(delta as u32);
                                }else{
                                    self.sel_date.year = self.sel_date.year.saturating_sub((-delta) as u32);
                                }
                            },
                            UpDownValue::Month => {
                                let max = 12;
                                let min = 1;
                                if delta >= 0{
                                    self.sel_date.month = self.sel_date.month.saturating_add(delta as u8);
                                }else{
                                    self.sel_date.month = self.sel_date.month.saturating_sub((-delta) as u8);
                                }
                                if self.sel_date.month > max{
                                    self.sel_date.month = 1
                                }else if self.sel_date.month < min{
                                    self.sel_date.month = 12
                                }
                            },
                        }
                    }
                    Buttons::SelDay(dia) => {
                        if dia > 0{
                            self.sel_date.day = dia as u8;
                        }
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.screen{
            Screens::Main => {
                column![
                    if let Some(data) = &self.last_afd_got{
                        text(format!("Ultimo AFD: {}", data.format("%d-%m-%Y %H:%M")))

                    }else{
                        text("PEGAR AFD!")
                    },
                    button("GetAFDFile")
                        .on_press(Message::ButtonPressed(Buttons::GetAFDFile)),
                    button("Calendario")
                        .on_press(Message::ButtonPressed(Buttons::SwitchTo(Screens::Calendar)))
                ].align_x(Center).into()
            },
            Screens::Calendar => {
                let mut dom: Column<Message> = column![text("Dom")].spacing(5).align_x(Center);
                let mut seg: Column<Message> = column![text("Seg")].spacing(5).align_x(Center);
                let mut ter: Column<Message> = column![text("Ter")].spacing(5).align_x(Center);
                let mut qua: Column<Message> = column![text("Qua")].spacing(5).align_x(Center);
                let mut qui: Column<Message> = column![text("Qui")].spacing(5).align_x(Center);
                let mut sex: Column<Message> = column![text("Sex")].spacing(5).align_x(Center);
                let mut sab: Column<Message> = column![text("Sab")].spacing(5).align_x(Center);
                let dias = self.sel_date.get_month_day_weekday();
                let first_weekday = dias.first().map(|(weekday, _)| *weekday);
                match first_weekday{
                        Some(Weekday::Sun) => (),
                        Some(Weekday::Mon) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                        },
                        Some(Weekday::Tue) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            seg = seg.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))))
                        },
                        Some(Weekday::Wed) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            seg = seg.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            ter = ter.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                        },
                        Some(Weekday::Thu) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            seg = seg.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            ter = ter.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            qua = qua.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                        },
                        Some(Weekday::Fri) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            seg = seg.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            ter = ter.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            qua = qua.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            qui = qui.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                        },
                        Some(Weekday::Sat) => {
                            dom = dom.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            seg = seg.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            ter = ter.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            qua = qua.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            qui = qui.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                            sex = sex.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(0))));
                        },
                        None => println!("Não funcionou")
                        }
                for (weekday, day) in dias {

                    let day_button = if self.sel_date.day as u32 == day{
                        button(text(format!("{} *",day.to_string())).color(Color::from_rgb(1.0, 0.0, 0.0))).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(day)))
                    }else{
                        button(text(day.to_string())).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::SelDay(day)))
                    };
                    match weekday {
                        Weekday::Sun => dom = dom.push(day_button),
                        Weekday::Mon => seg = seg.push(day_button),
                        Weekday::Tue => ter = ter.push(day_button),
                        Weekday::Wed => qua = qua.push(day_button),
                        Weekday::Thu => qui = qui.push(day_button),
                        Weekday::Fri => sex = sex.push(day_button),
                        Weekday::Sat => sab = sab.push(day_button),
                    }
                }
                let mut acontecimentos: Column<Message> = column![
                    row![
                        text("ACONTECIMENTOS").size(25.0).color(Color::from_rgb(0.5, 0.5, 0.5))
                    ]
                ];
                column![
                    row![
                        button(text("Voltar")).on_press(Message::ButtonPressed(Buttons::SwitchTo(Screens::Main)))
                    ].spacing(10),
                    column![
                        row![
                            text("CALENDARIO").size(30.0).color(Color::from_rgb(0.5, 0.5, 0.5))
                        ].spacing(10),
                        Space::with_height(Length::Fixed(15.0)),
                        row![
                            text(format!("{}, {} de",Self::weekday_pt(self.sel_date.get_week_day()), self.sel_date.day)),
                            button("<-").on_press(Message::ButtonPressed(Buttons::UpDownButton(-1, UpDownValue::Month))),
                            text(format!("{}", Self::int_to_month_pt(self.sel_date.month).unwrap().to_string())),
                            button("->").on_press(Message::ButtonPressed(Buttons::UpDownButton(1, UpDownValue::Month))),
                            text("de"),
                            button("<-").on_press(Message::ButtonPressed(Buttons::UpDownButton(-1, UpDownValue::Year))),
                            text(format!("{}", self.sel_date.year)),
                            button("->").on_press(Message::ButtonPressed(Buttons::UpDownButton(1, UpDownValue::Year))),
                        ].spacing(10).align_y(Center),
                        Space::with_height(Length::Fixed(15.0)),
                        row![
                            dom,
                            seg,
                            ter,
                            qua,
                            qui,
                            sex,
                            sab,
                        ].spacing(10),
                        Space::with_height(Length::Fixed(25.0)),
                        scrollable(container(acontecimentos).align_x(Center).width(Fill)).height(Fill),
                    ].width(Fill).height(Fill).align_x(Center)
                ].into()
            }
        }
    }

}


fn main() -> iced::Result{
    dotenv::dotenv().ok();
    iced::application("Interface RH", InterfaceRH::update, InterfaceRH::view)
        .window_size((800.0, 600.0))
        .run()
}
// fn main() {
// }
