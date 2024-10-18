extern crate getopts;
use getopts::Matches;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

// структура хранящая количество объектов в файле
struct FileObjects {
    chars: u64,
    lines: u64,
    words: u64,
}

impl FileObjects {
    // конструктор
    fn new() -> FileObjects {
        FileObjects {
            chars: 0,
            lines: 0,
            words: 0,
        }
    }
    //функция для суммирования значений текущего экземпляра с другим
    fn add(&mut self, summand: FileObjects) {
        self.chars += summand.chars;
        self.lines += summand.lines;
        self.words += summand.words;
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // используем функционал библиотеки "getopts" для парсинга флагов

    // создаем переменную хранящую ожидаемые флаги
    let mut opts = Options::new();

    // добавляем флаги -c -l -w
    opts.optflag("c", "", "print the number of characters in the file");
    opts.optflag("l", "", "print the number of lines in the file");
    opts.optflag("w", "", "display the number of words in the file");

    // парсим флаги из массива аргументов, в случае получения неверного флага выводим ошибку
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            println!("invalid option");
            return;
        }
    };

    let flags = matches.clone();

    // если указаны свободные аргументы (в нашем случае имена файлов) проводим подсчет обьектов и выводим в терминал
    if !matches.free.is_empty() {
        // переменная хранящая общее количество обьектов для всех указанных файлов
        let mut total = FileObjects::new();

        // получаем строки хранящую имя фалйа
        for file in &matches.free {
            // выполняем сканирование файла и записываем результат
            let objects = match file_checker(file.as_str(), &flags) {
                Ok(obj) => obj,
                Err(_) => {
                    //обрабатываем ситуацию когда указан неверный путь к файлу
                    println!("{}: No such file or directory", file);
                    continue;
                }
            };

            // выводим информацию в зависимости от указанных флагов
            if flags.opt_present("l") {
                print!(" {}", objects.lines);
            }
            if flags.opt_present("w") || no_flags_present(&flags) {
                print!(" {}", objects.words);
            }
            if flags.opt_present("c") {
                print!(" {}", objects.chars);
            }
            print!(" {}\n", file);

            // прибавляем полученные значения к общему количеству обьектов для всех указанных файлов
            total.add(objects);
        }
        // если указано более одного файла, выводим информацию об общем количестве обьектов
        if matches.free.len() > 1 {
            if flags.opt_present("l") {
                print!(" {}", total.lines);
            }
            if flags.opt_present("w") || no_flags_present(&flags) {
                print!(" {}", total.words);
            }
            if flags.opt_present("c") {
                print!(" {}", total.chars);
            }
            println!(" total");
        }
    } else {
        print_usage(&program, opts);
        return;
    };
}

// функция получения количества обьектов для указанного файла
fn file_checker(file_name: &str, flags: &Matches) -> Result<FileObjects, Error> {
    let mut objects = FileObjects::new();

    let input = File::open(file_name)?; // открываем указанный файл
    let reader = BufReader::new(input); // сохраняем его в буфер

    // получаем итератор по строкам файла и проходим по каждой из них
    for line in reader.lines() {
        let line = line.unwrap();

        // прибавляем количество символов в строке
        if flags.opt_present("c") {
            objects.chars += line.len() as u64 + 1;
        }

        // прибавляем единицу к количеству строк
        if flags.opt_present("l") {
            objects.lines += 1;
        }

        // прибавляем количество слов в строке если установлен флаг "-w" или не установлен ни один из флагов
        if flags.opt_present("w") || no_flags_present(flags) {
            objects.words += line.clone().split_whitespace().count() as u64;
        }
    }
    Ok(objects)
}

// функция возвращает true если не указано ни одного флага
fn no_flags_present(matches: &Matches) -> bool {
    !matches.opt_present("w") && !matches.opt_present("l") && !matches.opt_present("c")
}
