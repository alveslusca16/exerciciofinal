use std::io;

fn main() {
    let mut alunos: Vec<String> = Vec::new();
    let mut medias: Vec<f32> = Vec::new();
    for _ in 0..5 {
        let mut nome = String::new();
        println!("Digite o nome do aluno: ");
        io::stdin().read_line(&mut nome).expect("Erro ao ler aluno");

        let mut nota1= String::new();
        println!("Digite sua primeira nota: ");
        io::stdin().read_line(&mut nota1).expect("Erro ao ler a nota");
        let nota1: f32 = nota1.trim().parse().expect("Erro ao converter a primeira nota");

        let mut nota2= String::new();
        println!("Digite sua segunda nota: ");
        io::stdin().read_line(&mut nota2).expect("Erro ao ler a nota");
        let nota2: f32 = nota2.trim().parse().expect("Erro ao converter a segunda nota");

        let mut nota3= String::new();
        println!("Digite sua terceira nota: ");
        io::stdin().read_line(&mut nota3).expect("Erro ao ler a nota");
        let nota3: f32 = nota3.trim().parse().expect("Erro ao converter a terceira nota");

        let media = (nota1 + nota2 + nota3)/3.0;

        alunos.push(nome.to_lowercase());
        
        medias.push(media);
    }

    loop {
        let mut buscador = String::new();
        println!("Informe o nome do aluno: ");
        io::stdin().read_line(&mut buscador).expect("Erro ao ler alunos");

        let buscador = buscador.trim().to_lowercase();

        let posicao = alunos.iter().position(|aluno| aluno.trim() == buscador);

        match posicao {
            Some(index) => {
                let media_aluno = medias[index];
                println!("A média do aluno {} é {}", alunos[index], media_aluno);
            }
            None => {
                println!("Aluno não encontrado");
            }
        }

        let mut resp = String::new();
        println!("Deseja continuar? S/N");
        io::stdin().read_line(&mut resp).expect("Erro ao ler a mensagem");

        if resp.trim().to_lowercase() != "s" {
            break;
        } 
    }
}
