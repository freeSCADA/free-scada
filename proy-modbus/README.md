# Concentrador MODBUS (Inserte nombre creativo para el proyecto)
Proyecto para presentar en el Seminario de ANDE 2025.

## Convenciones

### Sobre la estructura general del proyecto
El proyecto utiliza Cargo workspaces para una mejor organización y escalabilidad del mismo.
Cargo soporta la creación de paquetes (packages) tanto binarios (ejecutables), como de librerías.

#### Para añadir un nuevo package binario:
Ejecute en su terminal:
```shell
cargo new --bin <binary_package>
```

#### Para añadir un nuevo package de librería:
Ejecute en su terminal:

```shell
cargo new --lib <library_package>
```

### Sobre el modo de añadir una nuevos elementos al proyecto.
Se deberá crear una nueva rama en el repositorio. Para ello ejecute los siguientes comandos:
```shell
git checkout -b nnnnnn/ccc-xxxx
```

Donde:
+ **nnnnnn** corresponde al **número de personal de ANDE** del responsable de dicha rama.
+ **ccc** corresponde a la acción que se desea realizar en general. Éstas pueden ser:
    - **fix** => arreglar algo.
    - **add** => añadir algo que no existía al proyecto.
    - **doc** => documentar alguna parte del proyecto.
    - **mod** => modificar algo dentro del proyecto.
+ **xxxx** corresponde al cambio concreto a realizarse, sin espacios y con guiones medios ("-")

## Objetivos
1. Construir un concentrador MODBUS.
2. Ganar el concurso.
3. Terminar mejor que Rody y Sebas :b