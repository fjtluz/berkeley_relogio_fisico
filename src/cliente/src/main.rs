use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use rand::{Rng, thread_rng};

fn format_time(time: i32) -> String {
    return if time > 9 { format!("{time}") } else { format!("0{time}") }
}

fn handle_client(mut stream: TcpStream, porta: u16, tempo_cliente_em_minutos: &mut i32) -> bool {
    let mut ajuste_feito = false;
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(tamanho) => {
            let mut resposta_servidor = String::new();

            for i in 0..tamanho {
                let char = char::from(buffer[i]);
                resposta_servidor.push(char);
            }

            if resposta_servidor.contains(":") {
                let digitos: Vec<&str> = resposta_servidor.split(":").collect();
                let horas: i32 = digitos[0].parse().unwrap();
                let minutos: i32 = digitos[1].parse().unwrap();

                let tempo_servidor_em_minutos = horas * 60 + minutos;
                let diferenca = *tempo_cliente_em_minutos - tempo_servidor_em_minutos;


                let connection = TcpStream::connect("localhost:1000");
                match connection {
                    Ok(mut stream) => {
                        let stream_body = format!("{}#{}", porta, diferenca);
                        stream.write(&stream_body.into_bytes()).expect("Não foi possível escrever para este servidor");
                    },
                    Err(_) => println!("Não foi possível se conectar ao servidor 1000"),
                }
            } else {
                let ajuste: i32 = resposta_servidor.parse().unwrap();

                *tempo_cliente_em_minutos += ajuste;

                ajuste_feito = true;
            }
        }
        Err(_) => println!("Incapaz de ler!"),
    }
    return ajuste_feito;
}

fn main() -> std::io::Result<()> {

    let mut rng = thread_rng();

    let (mut hora_cliente, mut minuto_cliente) = (rng.gen_range(0..24), rng.gen_range(0..60));
    let mut tempo_cliente_em_minutos = hora_cliente * 60 + minuto_cliente;
    let time_now = format!("{}:{}", format_time(hora_cliente), format_time(minuto_cliente));

    let localhost = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));

    let addrs: [SocketAddr; 5] = [
        SocketAddr::from((localhost, 1001)),
        SocketAddr::from((localhost, 1002)),
        SocketAddr::from((localhost, 1003)),
        SocketAddr::from((localhost, 1004)),
        SocketAddr::from((localhost, 1005)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();

    let listener_port = listener.local_addr().unwrap().port();
    println!("Cliente com porta: {}", listener_port);

    println!("Tempo cliente: {}", time_now);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ajuste_feito = handle_client(stream, listener_port,&mut tempo_cliente_em_minutos);

                if ajuste_feito {
                    hora_cliente = tempo_cliente_em_minutos / 60;
                    minuto_cliente = tempo_cliente_em_minutos % 60;

                    if hora_cliente < 0 {
                        hora_cliente *= -1;
                    }

                    if minuto_cliente < 0 {
                        minuto_cliente *= -1
                    }

                    let time_now = format!("{}:{}", format_time(hora_cliente), format_time(minuto_cliente));

                    println!("Novo tempo cliente: {time_now}");
                }

            },
            Err(_) => break,
        }
    }

    return Ok(())
}