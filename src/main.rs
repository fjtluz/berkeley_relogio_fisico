use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use rand::{Rng, thread_rng};

fn handle_client(mut stream: TcpStream, diffs: &mut Vec<(u16, i32)>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let mut client_response = String::new();

            for i in 0..size {
                let char = char::from(buffer[i]);
                client_response.push(char);
            }

            let broken_down = client_response.split("#").collect::<Vec<&str>>();

            diffs.push((broken_down[0].parse().unwrap(), broken_down[1].parse().unwrap()));
        },
        Err(_) => println!("Incapaz de ler!")
    }
}

fn formata_tempo(tempo: i32) -> String {
    return if tempo > 9 { format!("{}", tempo) } else { format!("0{}", tempo) };
}

fn main() -> std::io::Result<()> {

    let mut rng = thread_rng();

    let (mut hora_servidor, mut minuto_servidor) = (rng.gen_range(0..25), rng.gen_range(0..60));

    let tempo_formatado = format!("{}:{}", formata_tempo(hora_servidor), formata_tempo(minuto_servidor));

    println!("Tempo servidor: {}", tempo_formatado);

    let mut diferencas: Vec<(u16, i32)> = vec![(1000, 0)];
    let mut respostas_cliente: usize = 0;

    let localhost = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));

    match TcpListener::bind(SocketAddr::new(localhost, 1000)) {
        Ok(servidor) => {
            for porta_cliente in 1001..=1005 {

                let endereco_cliente = SocketAddr::new(localhost, porta_cliente);

                let conexao = TcpStream::connect(endereco_cliente);
                match conexao {
                    Ok(mut stream) => {
                        stream.write(tempo_formatado.as_bytes()).expect("Não foi possível escrever para este client");
                        respostas_cliente += 1;
                    },
                    Err(_) => println!("Não foi possível se conectar ao cliente {}", porta_cliente)
                }
            }

            for requisicao in servidor.incoming() {
                match requisicao {
                    Ok(stream) => {
                        handle_client(stream, &mut diferencas);
                        if diferencas.len() == respostas_cliente + 1 {

                            let mut media = 0;
                            for diferenca in &diferencas {
                                media += diferenca.1;
                            }
                            media /= diferencas.len() as i32;
                            let mut tempo_em_minutos = (hora_servidor * 60) + minuto_servidor;
                            tempo_em_minutos += media;

                            hora_servidor = tempo_em_minutos / 60;
                            minuto_servidor = tempo_em_minutos % 60;

                            if hora_servidor < 0 {
                                hora_servidor *= -1;
                            }

                            if minuto_servidor < 0 {
                                minuto_servidor *= -1
                            }

                            let tempo_formatado = format!("{}:{}", formata_tempo(hora_servidor), formata_tempo(minuto_servidor));

                            println!("Novo tempo servidor: {}", tempo_formatado);

                            for (porta, diferenca) in &diferencas {
                                if *porta == 1000 {
                                    continue;
                                }
                                let endereco_cliente = format!("localhost:{}", porta);

                                let ajuste = media + (diferenca * -1);
                                // println!("Porta:{}; Média: {}; Diferença: {}; Ajuste: {}", porta, media, diferenca, ajuste);

                                let conexao = TcpStream::connect(endereco_cliente);
                                match conexao {
                                    Ok(mut stream) => {
                                        stream.write(ajuste.to_string().as_bytes()).expect("Não foi possível escrever para este client");
                                    },
                                    Err(_) => println!("Não foi possível se conectar ao cliente {}", porta)
                                }
                            }
                        }
                    },
                    Err(_) => break,
                }
            }
        }
        Err(_) => println!("Porta 1000 já está em uso"),
    }

    return Ok(())
}
