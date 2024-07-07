/*use sqlx::{sqlite::SqliteRow, Row};*/
use sqlite::{Connection, Result, State, Value}; //sqlite = "0.34.0"
//use rusqlite::{Connection, Result}; //rusqlite = "0.31.0"
use crate::models::Project;
use crate::models::Author;
use std::convert::TryFrom;
/*
// Assuming row.read::<&str, _>(col) returns a reference to a &str
pub fn read_string(row: &sqlite::Row, col: &str) -> Result<String, sqlite::Error> {
    let value: &Value = row.read(col)?;
    let s = String::try_from(value)?;
    Ok(s)
}*/
// Функция для создания таблицы и вставки данных в базу данных
//pub fn insert_projects(connection: &Connection) -> Result<()> {
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE_CONNECTION: Arc<Mutex<Option<Connection>>> = Arc::new(Mutex::new(None));
}

pub async fn insert_projects() -> Result<()> {

    let flags = sqlite::OpenFlags::new().with_create().with_read_write();
    let connection = Connection::open_with_flags(":memory:", flags).unwrap();

    // Устанавливаем глобальную переменную DATABASE_CONNECTION
    let mut database_connection = DATABASE_CONNECTION.lock().unwrap();
    *database_connection = Some(connection);


    
    let query = "
    CREATE TABLE topic (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        theme TEXT NOT NULL
    );
    INSERT INTO topic (theme) VALUES ('Экология');
    INSERT INTO topic (theme) VALUES ('Искусство');
    INSERT INTO topic (theme) VALUES ('Образование');
    INSERT INTO topic (theme) VALUES ('Здравоохранение');
    INSERT INTO topic (theme) VALUES ('Социальные проекты');

    CREATE TABLE author (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        nickname TEXT
    );
    
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Анна', 'Иванова', 'rm_ann_ivanova.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Дмитрий', 'Смирнов', 'rm_dmitry_smirnov.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Елена', 'Петрова', 'rm_elena_petrova.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Александр', 'Козлов', 'rm_alex_kozlov.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Мария', 'Сидорова', 'rm_maria_sidorova.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Иван', 'Федоров', 'rm_ivan_fedorov.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Анастасия', 'Новикова', 'rm_anastasia_novikova.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Павел', 'Кузнецов', 'rm_pavel_kuznetsov.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Екатерина', 'Соколова', 'rm_ekaterina_sokolova.testnet');
    INSERT INTO author (first_name, last_name, nickname) VALUES ('Сергей', 'Иванов', 'rm_sergey_ivanov.testnet');



    CREATE TABLE IF NOT EXISTS projects (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author INTEGER NOT NULL,
        name TEXT NOT NULL,
        topic INTEGER,
        description TEXT NOT NULL,
        file_path TEXT,
        date_published TEXT NOT NULL,
        FOREIGN KEY (topic) REFERENCES topic(id),
        FOREIGN KEY (author) REFERENCES author(id)
    );

    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Искусство для всех', 2, 'https://museum-design.ru/wp-content/uploads/modern-art-by-denimu2.jpg', 'Использование искусства для решения социальных проблем и вдохновения на изменения. Цель проекта - создание вдохновляющих и обсуждаемых художественных работ, способствующих позитивным изменениям в обществе. Мы собираемся организовать мастер-классы и выставки.', 2, '2023-06-20 14:45:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Технологии для образования', 3, 'http://проф-обр.рф/_bl/12/91795848.jpg', 'Повышение доступности образования с помощью технологий. Цель проекта - разработка интерактивных образовательных инструментов, доступных учащимся по всему миру. Мы собираемся создать онлайн-платформу для обмена знаниями и навыками.', 2, '2023-08-10 12:00:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Инициатива по здравоохранению', 4, 'https://kaknaladoni.de/wp-content/uploads/vrach-patient-1210x642.jpg', 'Улучшение доступа к здравоохранению и его результатов путем инноваций. Цель проекта - разработка доступных и масштабируемых решений в области здравоохранения для пользы сообществ по всему миру. Мы собираемся создать онлайн-платформу для консультаций и обмена опытом.', 5, '2023-10-05 09:20:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Проект по городскому садоводству', 5, 'https://art-fresco.ru/upload/webp/iblock/4de/v60rgr1uflw0gcdgqsmodgqrhfp2uiuo.webp', 'Содействие устойчивому сельскому хозяйству и продовольственной безопасности через городское садоводство. Цель проекта - развитие местного производства, укрепление продовольственной устойчивости и укрепление связи с природой. Мы собираемся организовать мастер-классы и выращивание овощей в городских условиях.', 6, '2023-12-30 16:55:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Революция в области возобновляемой энергии', 1, 'http://reform.energy/uploads/Veter,%20solnce%20&%20co/solar_wind_1-889x592.jpg', 'Переход к будущему с возобновляемой энергией через совместные инновации. Цель проекта - разработка чистых энергетических решений для борьбы с изменением климата и создания устойчивого мира. Мы собираемся организовать кампании по энергосбережению и разработать альтернативные источники энергии.', 5, '2023-03-08 14:10:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Инициатива по концептуальному дизайну', 2, 'https://img.freepik.com/premium-photo/concept-design-style-minimalist-building_547280-2263.jpg?w=740', 'Создание доступных сред для всех через концептуальный дизайн. Цель проекта - разработка продуктов и услуг, которые учитывают разнообразие потребностей и способностей пользователей. Мы собираемся провести исследования и разработать руководства по концептуальному дизайну.', 4, '2023-02-15 16:30:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Благотворительный проект \"Подари улыбку\"', 3, 'http://nko-karelia.ru/Attachments/Companies/2148/logo/8521-medium.jpg', 'Поддержка детей и молодежи из малообеспеченных семей. Цель проекта - предоставление образовательных, культурных и спортивных возможностей для детей, которые в них нуждаются. Мы собираемся собирать пожертвования и организовывать встречи с волонтёрами.', 3, '2023-11-20 11:45:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Проект \"Зеленый город\"', 4, 'https://wsjournal.ru/wp-content/uploads/2017/06/zelenyj-gorod-budushhee-nashej-planety6-1038x576.jpg', 'Преобразование городских пространств в зеленые оазисы для здоровья и благополучия. Цель проекта - создание и улучшение общественных зон с зелеными насаждениями для обогащения городской среды. Мы собираемся проводить акции по озеленению и обучать местных жителей уходу за растениями.', 2, '2023-09-25 10:00:00');
    INSERT INTO projects (name, topic, file_path, description, author, date_published)
        VALUES ('Кулинарный проект \"Вкусная традиция\"', 5, 'https://www.matrony.ru/wp-content/uploads/2015/12/Stollen_with_candied_fruits.jpg', 'Сохранение и продвижение культурных кулинарных традиций различных народов. Цель проекта - организация мастер-классов, кулинарных фестивалей и создание кулинарных книг для сохранения и передачи кулинарного наследия. Мы собираемся провести исследования и собрать рецепты блюд из разных культур.', 1, '2023-07-12 09:00:00');
    ";

    
    if let Some(conn) = &*database_connection {
        // Разблокировка MutexGuard и получение доступа к Connection
        //let connection: Connection = (Arc::clone(conn)).as_ref();
        println!("table was created!");
        conn.execute(query);
        // Теперь вы можете использовать это соединение для выполнения запросов к базе данных
   } else {
        println!("Ошибка: соединение с базой данных не установлено");
    }
    Ok(())
    
    /*
    connection.execute(
        query,
        (), // empty list of parameters.
    )
    */
}

pub fn insert_author(author: String, connection: &mut Connection ) -> i32 {
        //let mut stmt = connection.prepare("SELECT * FROM projects")?;
        let mut author_id: i32 = -1;
        println!("insert author... --> {:?}", author);
    
    
            let query = format!("
            INSERT INTO author (first_name, last_name, nickname)
            VALUES ('default', 'default', '{}');", author);
            println!("{}", query);
            connection.execute(query);  

            let query = format!("SELECT id FROM author WHERE nickname = '{}';", author);
            println!("{}", query);
    
            connection
            .iterate(query, |rows| {

                author_id = rows.get(0).unwrap().1.unwrap().parse().unwrap();
                true
            })
            .unwrap();
            println!("author id --> {:?}", author_id);
        author_id
    }

pub fn insert_topic(theme: String, connection: &mut Connection) -> i32 {
        //let mut stmt = connection.prepare("SELECT * FROM projects")?;
        let mut topic_id: i32 = -1;
        println!("insert topic... --> {:?}", theme);
    
    
            let query = format!("INSERT INTO topic (theme) VALUES ('{}'); ", theme);
            println!("{}", query);
            connection.execute(query);      
    
            let query = format!("SELECT id FROM topic WHERE theme = '{}';", theme);
            println!("{}", query);
            connection
            .iterate(query, |rows| {

                topic_id = rows.get(0).unwrap().1.unwrap().parse().unwrap();
                true
            })
            .unwrap();
            println!("topic id --> {:?}", topic_id);
        topic_id
    }

pub fn add_project(project: Project) -> Result<()> {
    println!("add projects...");
    // Получаем глобальное соединение с базой данных
    let mut database_connection = DATABASE_CONNECTION.lock().unwrap();
    println!("{:?}", project);
    // Проверяем, установлено ли соединение
    if let Some(conn) = &mut *database_connection {

        let author_id = insert_author(project.author, conn);
        let topic_id = insert_topic(project.topic, conn);

        // Создаем запрос для вставки проекта в таблицу
        let query = format!("
            INSERT INTO projects (name, topic, file_path, description, author, date_published)
            VALUES ('{}', {}, '{}', '{}', {}, '{}');
        ", project.name, topic_id, project.file_path, project.description, author_id, project.date_published);
        
        println!("{}", query);
        // Выполняем запрос
        let _is = conn.execute(query);
        
        println!("Project inserted successfully");
    } else {
        println!("Ошибка: соединение с базой данных не установлено");
    }

    Ok(())
}





/*
// Функция для выборки данных из базы данных
pub fn test_select_projects(connection: &Connection) -> Result<()> {
    let query = "SELECT * FROM projects";

    connection.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    })
}
*/
/*
pub fn select_projects(connection: &Connection) -> Result<Vec<Project>> {
    let mut projects = Vec::new();

    // Подготовка запроса
    let mut stmt = connection.prepare("SELECT * FROM projects")?;
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get("id")?,
            name: row.get("name")?,
            type_: row.get("type")?,
            file_path: row.get("file_path")?,
            description: row.get("description")?,
            author: row.get("author")?,
            date_published: row.get("date_published")?,
        })
    })?;

    // Итерация по результатам запроса и добавление проектов в вектор
    for project in rows {
        projects.push(project?);
    }

    Ok(projects)
}
*/



pub fn select_projects() -> Result<Vec<Project>> {
    //let mut stmt = connection.prepare("SELECT * FROM projects")?;
    let mut projects: Vec<Project> = Vec::new();

    let database_connection = DATABASE_CONNECTION.lock().unwrap();
    if let Some(connection) =  &*database_connection {

        let query = ("SELECT projects.id, name, theme, file_path, description, 
        nickname, date_published FROM projects
        JOIN author ON author.id=projects.author
        JOIN topic ON topic.id=projects.topic     
        ;");

        /*
SELECT projects.name, topic.name, file_path, description, 
        nickname, date_published FROM projects
        JOIN topic ON topic.id=projects.topic
        JOIN author ON author.id=projects.author;
         */

        connection
        .iterate(query, |rows| {
    
            let mut data : Vec<Project> = Vec::new();
            /*for &(name, value) in rows.iter() {
                //data_proj.push();
                // println!("{} = {} = {} = {}", rows.get(0).unwrap().0, rows.get(0).unwrap().1.unwrap(), rows.get(1).unwrap().0, rows.get(1).unwrap().1.unwrap());
                //println!("{} = {}", name, value.unwrap());
            }*/
            let project = Project {
                id: rows.get(0).unwrap().1.unwrap().parse().unwrap(),
                name: rows.get(1).unwrap().1.unwrap().to_string(),
                topic: rows.get(2).unwrap().1.unwrap().to_string(),
                file_path: rows.get(3).unwrap().1.unwrap().to_string(),
                description: rows.get(4).unwrap().1.unwrap().to_string(),
                author : rows.get(5).unwrap().1.unwrap().to_string(),
                date_published: rows.get(6).unwrap().1.unwrap().to_string()
            };
    
            projects.push(project);
            true
        })
        .unwrap();

   } else {
        println!("Ошибка: соединение с базой данных не установлено");
    }

    Ok(projects)
}


/*
pub fn get_author() -> Author {
    //let mut stmt = connection.prepare("SELECT * FROM projects")?;
    let mut projects: Vec<Project> = Vec::new();

    let database_connection = DATABASE_CONNECTION.lock().unwrap();
    if let Some(connection) =  &*database_connection {

        let query = ("SELECT projects.id, name, theme, file_path, description, 
        nickname, date_published FROM projects
        JOIN author ON author.id=projects.author
        JOIN topic ON topic.id=projects.topic     
        ;");

        /*
SELECT projects.name, topic.name, file_path, description, 
        nickname, date_published FROM projects
        JOIN topic ON topic.id=projects.topic
        JOIN author ON author.id=projects.author;
         */

        connection
        .iterate(query, |rows| {
    
            let mut data : Vec<Project> = Vec::new();
            /*for &(name, value) in rows.iter() {
                //data_proj.push();
                // println!("{} = {} = {} = {}", rows.get(0).unwrap().0, rows.get(0).unwrap().1.unwrap(), rows.get(1).unwrap().0, rows.get(1).unwrap().1.unwrap());
                //println!("{} = {}", name, value.unwrap());
            }*/
            let project = Project {
                id: rows.get(0).unwrap().1.unwrap().parse().unwrap(),
                name: rows.get(1).unwrap().1.unwrap().to_string(),
                topic: rows.get(2).unwrap().1.unwrap().to_string(),
                file_path: rows.get(3).unwrap().1.unwrap().to_string(),
                description: rows.get(4).unwrap().1.unwrap().to_string(),
                author : rows.get(5).unwrap().1.unwrap().to_string(),
                date_published: rows.get(6).unwrap().1.unwrap().to_string()
            };
    
            projects.push(project);
            true
        })
        .unwrap();

   } else {
        println!("Ошибка: соединение с базой данных не установлено");
    }

    Ok(projects)
}
*/