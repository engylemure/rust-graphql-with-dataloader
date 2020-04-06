-- Your SQL goes here
CREATE TABLE movies (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    released_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO movies (id, name, released_at)
    VALUES
    (1, 'Episode I - The Phantom Menace', '1999-06-19 00:00:01'),
    (2, 'Episode II - Attack of the Clones', '2002-06-16 00:00:01'),
    (3, 'Episode III - Revenge of the Sith', '2006-06-19 00:00:01'),
    (4, 'Episode IV - A New Hope', '1977-06-26 00:00:01'),
    (5, 'Episode V - The Empire Strikes Back', '1980-06-21 00:00:01'),
    (6, 'Episode VI - Return of the Jedi', '1983-06-26 00:00:01'),
    (7, 'Episode VII - The Force Awakens', '2016-12-18 00:00:01'),
    (8, 'Episode VIII - The Last Jedi', '2017-12-16 00:00:01'),
    (9, 'Episode IX - The Rise of Skywalker', '2019-12-20 00:00:01');


CREATE TABLE characters (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(266) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO characters (id, name)
    VALUES
    (1, 'Luke Skywalker'),
    (2, 'Han Solo'),
    (3, 'Princess Leia'),
    (4, 'Obi-Wan Kenobi'),
    (5, 'Anakin Skywalker'),
    (6, 'Darth Vader'),
    (7, 'Kylo Ren'),
    (8, 'Rey'),
    (9, 'Finn'),
    (10, 'Poe Dameron'),
    (11, 'R2-D2'),
    (12, 'C-3PO'),
    (13, 'Yoda'),
    (14, 'Leia Organa');

CREATE TABLE movie_characters (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    movie_id INTEGER NOT NULL,
    character_id INTEGER NOT NULL,
    KEY `movie_characters_fk1` (`movie_id`),
    KEY `movie_characters_fk2` (`character_id`),
    CONSTRAINT `movie_characters_fk1` FOREIGN KEY (`movie_id`) REFERENCES `movies` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `movie_characters_fk2` FOREIGN KEY (`character_id`) REFERENCES `characters` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO movie_characters (`movie_id`, `character_id`)
    VALUES
    (1, 6),
    (1, 4),
    (1, 13),
    (1, 12),
    (1, 11),
    (2, 6),
    (2, 4),
    (2, 13),
    (2, 12),
    (2, 11),
    (3, 6),
    (3, 4),
    (3, 13),
    (3, 12),
    (3, 11),
    (3, 1),
    (3, 14),
    (4, 6),
    (4, 4),
    (4, 13),
    (4, 12),
    (4, 11),
    (4, 1),
    (4, 14),
    (4, 2),
    (5, 6),
    (5, 4),
    (5, 5),
    (5, 13),
    (5, 12),
    (5, 11),
    (5, 1),
    (5, 14),
    (5, 2),
    (6, 6),
    (6, 4),
    (6, 5),
    (6, 13),
    (6, 12),
    (6, 11),
    (6, 1),
    (6, 14),
    (6, 2);



