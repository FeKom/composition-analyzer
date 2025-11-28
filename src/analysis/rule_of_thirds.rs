
use image::{DynamicImage, GenericImage, GenericImageView};
//centro da imagem é altura// e largura dividida por 2
// os terços são divididos em 3 partes iguais tanto na altura quanto na largura
// para saber onde estao os pontos de interceção basta dividir a altura e largura por 3 e mutiplicar por 2

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

//nao é obrigatorio isso, mas é uma convenção legal. Serve para facilitar a criação de novos pontos
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct RuleOfThirds {
    pub score: f32,
    pub intersection_points: [Point; 4],
    pub elements_on_points: usize,
}


pub fn analyze_rule_of_thirds(img: &DynamicImage) -> RuleOfThirds {
    let (width, height) = img.dimensions();
    let third_width = width / 3;
    let third_height = height / 3;

    //isso aq é acessado pelo indice do array 0 top_left, 1 top_right, 2 bottom_left, 3 bottom_right
    let power_points = [
        Point::new(third_width, third_height),
        Point::new(2 * third_width, third_height),
        Point::new(third_width, 2 * third_height),
        Point::new(2 * third_width, 2 * third_height),
    ];

    //aqui é onde a mágica acontece, mas como eu não tenho um algoritmo de detecção de objetos implementado, vou deixar isso como 0 por enquanto
    RuleOfThirds {
        score: 0.0,
        intersection_points: power_points,
        elements_on_points: 0,
    }

}

pub fn draw_thirds_grid(img: &DynamicImage ,power_points: &[Point; 4]) -> DynamicImage {
    let mut img_with_grid = img.clone();
    let (width, height) = img.dimensions();

    // Desenhar linhas verticais
    let third_width = width / 3;
    for x in [third_width, 2 * third_width] {
        for y in 0..height {
            img_with_grid.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
    }

    // Desenhar linhas horizontais
    let third_height = height / 3;
    for y in [third_height, 2 * third_height] {
        for x in 0..width {
            img_with_grid.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
    }

    // Desenhar pontos de interseção
    for point in power_points.iter() {
        // Desenhar um ponto maior para melhor visibilidade
        for dx in 0..7 {
            for dy in 0..7 {
                // Desenhar um quadrado de 7x7 pixels
                let px = point.x + dx - 2;
                let py = point.y + dy - 2;
                // Garantir que os pixels estejam dentro dos limites da imagem
                if px < width && py < height {
                    img_with_grid.put_pixel(px, py, image::Rgba([0, 210, 0, 210]));
                }
            }
        }
    }

    img_with_grid
}

