//use std::fs::File;
//use std::io::Write;

use hound::{WavWriter, SampleFormat};

use image::{ImageBuffer, Luma};

// Кропаем изображение до радиуса
fn crop_image(image_crop: ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>>  {    
   
    let (width, height) = image_crop.dimensions();
    let threshold = 30u8;

    // Определяем начало и конец изображения по вертикали и горизонтали
    let mut min_x = width;
    let mut max_x = 0;
    let mut min_y = height;
    let mut max_y = 0;

    // Кропаем
    for y in 0..height {
        for x in 0..width {
            let pixel = image_crop.get_pixel(x, y).0[0];

            if pixel > threshold {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    let square_width = max_x - min_x + 1;
    let square_height = max_y - min_y + 1;
    let mut cropped_image = ImageBuffer::<Luma<u8>, Vec<u8>>::new(square_width, square_height);

    // Собираем
    for y in 0..square_height {
        for x in 0..square_width {
            let pixel = image_crop.get_pixel(min_x + x, min_y + y);
            cropped_image.put_pixel(x, y, *pixel);
        }
    }

    // Возвращаем
    Some(cropped_image)
}

fn main() {

    let input_image = "input.png";

    let image_to_crop = image::open(input_image).unwrap().to_luma8();
    let cropped_image = crop_image(image_to_crop).unwrap_or(ImageBuffer::new(1, 1));

    // Возвращаем кортеж с количеством пикселей по горизонтали и вертикали
    let (width, height) = cropped_image.dimensions();

    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;
    let spiral_radius = 300.0;
    let spiral_turns = 80.0; // Количество витков спирали
    let spiral_spacing = 8.13; // Расстояние между витками

    // Заголовок файла wav
    let spec = hound::WavSpec {
        channels: 1, // Один канал
        sample_rate: 44100, // Частота дискретизации 44100 Гц
        bits_per_sample: 8, // Разрешение звука 16 бит
        sample_format: SampleFormat::Int, // Формат данных в файле wav
    };

    // Создаем файл wav
    let mut writer = WavWriter::create("output.wav", spec).unwrap();
    // Создаем массив
    let mut pixel_values = Vec::new();

    // Обходим пиксели по спирали и добавляем их значения в вектор
    for t in (0..1500000).rev() {
        let angle = (t as f64 / 1500000.0) * spiral_turns * 2.0 * std::f64::consts::PI;
        let radius = (t as f64 / 1500000.0) * spiral_radius + (angle * spiral_spacing) / (2.0 * std::f64::consts::PI);

        let x = center_x + angle.cos() * radius;
        let y = center_y + angle.sin() * radius;

        if x >= 0.0 && x < width as f64 && y >= 0.0 && y < height as f64 {
            let pixel = cropped_image.get_pixel(x as u32, y as u32).0[0];
            pixel_values.push(255 - pixel);
            //cropped_image.put_pixel(x as u32, y as u32, white);
        }
    }
    //cropped_image.save(output_image).unwrap();

    // Конвертируем значения пикселей в значения сигнала в диапазоне от 0 до 65535
    let signal_values: Vec<i8> = pixel_values
        .clone()
        .into_iter()
        .map(|p| ((255 - p) as f32 / 255.0 * 128.0 - 128.0) as i8)
        .collect();

    // Открываем файл для записи
    //let mut file = File::create("output.txt").unwrap();

    // Записываем значения сигнала в файл wav
    for value in signal_values {
        writer.write_sample(value).unwrap();
    }

    // Завершаем запись в файл wav и сохраняем его
    writer.finalize().unwrap();
        println!("Аудио файл сохранен в output.wav");

    // Завершаем выполнение программы
    //println!("Значения пикселей записаны в файл output.txt");
}