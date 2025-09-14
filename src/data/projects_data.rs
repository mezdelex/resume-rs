use crate::models::project::Project;
use once_cell::sync::Lazy;

pub static PROJECTS: Lazy<Vec<Project>> = Lazy::new(|| {
    vec![
      Project {
        id: String::from("unpack"),
        pushed_at: String::from(""),
        name: String::from("UnPack"),
        image: String::from("./images/unpack.png"),
        repo: String::from("https://github.com/mezdelex/unpack"),
        app:String::from("https://github.com/mezdelex/unpack"),
        description: String::from("A minimal layer on top of vim.pack API to allow single file plugin configurations. vim.pack is currently under development and only available in the Neovim's nightly branch. Allows deferring plugin load and managing dependencies in a simple yet convenient way via commands or exposed module methods. For a more detailed documentation, check the repository."),
      },
      Project {
        id: String::from("CleanTemplate9"),
        pushed_at: String::from(""),
        name: String::from(".NET9 Clean Architecture Template"),
        image: String::from("./images/clean9.png"),
        repo: String::from("https://github.com/mezdelex/CleanTemplate8"),
        app: String::from("https://learn.microsoft.com/en-us/dotnet/architecture/modern-web-apps-azure/common-web-application-architectures"),
        description:
          String::from(".NET9 Clean Architecture template using DDD, Event Sourcing with MediatR library, Pub/Sub pattern with MassTransit library and RabbitMQ, Entity Framework Core's Identity, Redis cache, CQRS pattern, Specification pattern, Minimal API, Docker for SQLServer, RabbitMQ and WebApi containerization, FluentValidations, Unit of Work pattern, Serilog logging, CSharpier pre-commit hooks with Husky, Testing with xUnit and Moq, FluentAssertions, etc. Coded with Neovim"),
      },
      Project{
        id: String::from("AoC2023"),
        pushed_at: String::from(""),
        name: String::from("Advent of Code (2023)"),
        image: String::from("./images/aoc2023.png"),
        repo: String::from("https://github.com/mezdelex/AoC2023"),
        app: String::from("https://adventofcode.com/2023"),
        description:
          String::from("Algorithmic problem solving to achieve further Rust language proficiency with data structures, mutability and other language specific stuff. Also added CI pipeline via GitHub Actions to run testing jobs server side. Coded with Neovim."),
      },
      Project{
        id: String::from("RustExamples"),
        pushed_at: String::from(""),
        name: String::from("Rust Examples"),
        image: String::from("./images/rust.png"),
        repo: String::from("https://github.com/mezdelex/RustExamples"),
        app: String::from("https://www.rust-lang.org/"),
        description:
          String::from("Rust repository containing examples on various topics related to the language, such as concurrency, mutability, borrowing, lifetimes, data structures, etc. Work in progress. Coded with Neovim."),
      },
      Project{
        id: String::from("CleanTemplate"),
        pushed_at: String::from(""),
        name: String::from(".NET7 Clean Architecture Template"),
        image: String::from("./images/clean.png"),
        repo: String::from("https://github.com/mezdelex/CleanTemplate"),
        app: String::from("https://learn.microsoft.com/en-us/dotnet/architecture/modern-web-apps-azure/common-web-application-architectures"),
        description:
          String::from(".NET7 Clean Architecture template using DDD, Event Sourcing with MediatR library, Pub/Sub pattern with MassTransit library and RabbitMQ, CQRS pattern, Minimal API, Docker for PostgreSQL and RabbitMQ containerization, FluentValidations, Unit of Work pattern, Serilog logging, EditorConfig, Testing with xUnit, FluentAssertions, etc. Coded with Neovim"),
      },
      Project{
        id: String::from("TodoApp"),
        pushed_at: String::from(""),
        name: String::from("TodoApp API"),
        image: String::from("./images/todoapp.png"),
        repo: String::from("https://github.com/mezdelex/TodoApp"),
        app: String::from("https://github.com/users/mezdelex/projects/1"),
        description:
          String::from("RESTful API to learn Go using Fiber framework, following Clean Architecture principles with Domain Driven Design in mind, JWT authentication/authorization, GORM as ORM with automigrations and code first schema definition, PostgreSQL as database and GitHub Projects as task and time management platform. Coded with Neovim."),
      },
      Project{
        id: String::from("Botchy"),
        pushed_at: String::from(""),
        name: String::from("Botchy"),
        image: String::from("./images/botchy.png"),
        repo: String::from("https://github.com/mezdelex/Botchy"),
        app: String::from("https://discord.com/"),
        description:
          String::from("A Discord bot written in Python to facilitate the sharing of information between guild members of the mobile game Mobile Legends: Adventure. The bot needs to be run either locally or hosted in a dedicated server so the commands can be recognized by Discord."),
      },
      Project{
        id: String::from("WeztermPowerShellCoreConfig"),
        pushed_at: String::from(""),
        name: String::from("Wezterm"),
        image: String::from("./images/wezterm.png"),
        repo: String::from("https://github.com/mezdelex/WeztermPowerShellCoreConfig"),
        app: String::from("https://github.com/wez/wezterm"),
        description:
          String::from("Wezterm config with Cascadia Code NF variant, Powershell Core shell, custom prompt forked from patriksvensson's Oh My Posh theme, fuzzy finding via PSFzf, autocompletion via PSReadLine, fast directory navigation via zoxide, icon glyphs via Terminal Icons, Lazygit, Lazydocker, k9s, Yazi TUI file manager..."),
      },
      Project{
        id: String::from("NeovimConfig"),
        pushed_at: String::from(""),
        name: String::from("Neovim"),
        image: String::from("./images/neovim.png"),
        repo: String::from("https://github.com/mezdelex/NeovimConfig"),
        app: String::from("https://neovim.io/"),
        description:
          String::from("Fully functional daily driver Neovim config with LSP support, language server installation manager, debugger adapter, auto syntax highlighting parser installation, code actions, LLM integration through provided adapters, linting, suggestions, formatting, diagnostics, file search, live grep, file manager, git integration, code modification history, status line, autopairs, autotags..."),
      },
      Project{
        id: String::from("AoC2022"),
        pushed_at: String::from(""),
        name: String::from("Advent of Code (2022)"),
        image: String::from("./images/aoc2022.png"),
        repo: String::from("https://github.com/mezdelex/AoC2022"),
        app: String::from("https://adventofcode.com/2022"),
        description:
          String::from("Algorithmic problem solving to achieve further C# 11 language proficiency in declarative paradigm using LINQ, Lambdas and Expression-Bodied members. Also added CI pipeline via GitHub Actions to run testing jobs server side. Coded with VSCode."),
      },
      Project{
        id: String::from(""),
        pushed_at: String::from("2023-11-08T07:40:47Z"),
        name: String::from("Touch Typing"),
        image: String::from("./images/monkeytype.png"),
        repo: String::from("https://github.com/monkeytypegame/monkeytype"),
        app: String::from("https://monkeytype.com/"),
        description:
          String::from("A journey started in May 2022 to improve my touch typing skills and eventually increase my productivity. My current best mark is at 125 WPM with 100% accuracy & 90% consistency @ monkeytype 15s typing test."),
      },
      Project{
        id: String::from("Calculator"),
        pushed_at: String::from(""),
        name: String::from("Calculator"),
        image: String::from("./images/calculator.png"),
        repo: String::from("https://github.com/mezdelex/Calculator"),
        app: String::from("https://calculator-mezdelex.netlify.app"),
        description:
          String::from("A small project done with React 18 using hooks, functional components, context, context provider, props destructuring, children reserved word and kind of higher order component-ish approach to be able to reuse hook logic. The deployment has been done using GitHub's CD pipeline against Netlify. Coded with VSCode."),
      },
      Project{
        id: String::from("LeetCode"),
        pushed_at: String::from(""),
        name: String::from("LeetCode"),
        image: String::from("./images/leetcode.png"),
        repo: String::from("https://github.com/mezdelex/LeetCode"),
        app: String::from("https://leetcode.com/mezdelex"),
        description:
          String::from("LeetCode exercises to keep algorithmic problem solving fresh and improve my expertise with C++, C# and Rust using cassert, .NET's xUnit and Rustlang's built in library for unit testing. Also added CI pipelines for each language via GitHub Actions to run testing jobs server side. Coded with Neovim."),
      },
      Project{
        id: String::from("Resume"),
        pushed_at: String::from(""),
        name: String::from("Resume"),
        image: String::from("./images/resume.png"),
        repo: String::from("https://github.com/mezdelex/Resume"),
        app: String::from("https://www.mezdelex.com"),
        description:
          String::from("Vue 3 app to serve as an online portfolio to show my development experience. Used Vue 3 Composition API with Typescript. The deployment has been done using GitHub's CD pipeline against Netlify and hosted in a dedicated domain. Coded with Neovim."),
      },
      Project{
        id: String::from("Concesionario-SPA"),
        pushed_at: String::from(""),
        name: String::from("Concesionario"),
        image: String::from("./images/concesionario.jpg"),
        repo: String::from("https://github.com/mezdelex/Concesionario-SPA"),
        app: String::from("https://www.youtube.com/watch?v=iDaF9cqdtz0&ab_channel=Mezdelex"),
        description:
          String::from("Technical test using Spring Boot 2.4.5, Vue 3 Composition API with PrimeVUE as UI library and IBM's FileNet service as automated document manager. Coded with VSCode. The project was not deployed anywhere so the link it's a YouTube demo."),
      },
      Project{
        id: String::from("Recetario-SPA"),
        pushed_at: String::from(""),
        name: String::from("Recetario SPA"),
        image: String::from("./images/recetario.jpg"),
        repo: String::from("https://github.com/mezdelex/Recetario-SPA"),
        app: String::from("https://spa-recetario.netlify.app"),
        description:
          String::from("Vue 3 app to consume a RESTful API done with Spring Boot 2.4.5 and Java JDK 11+. Used PrimeVUE as UI library, Javascript ES6, Animate CSS, 2 way syncs, props, events, mixins, slots, etc. The deployment has been done using GitHub's CD pipeline against Netlify. Coded with VSCode. JawsDB service shutsdown the DB after 30 min of inactivity, so if you don't see any content let it boot."),
      },
      Project{
        id: String::from("Recetario-RESTful-API"),
        pushed_at: String::from(""),
        name: String::from("Recetario RESTful API"),
        image: String::from("./images/recetario2.jpg"),
        repo: String::from("https://github.com/mezdelex/Recetario-RESTful-API"),
        app: String::from("https://db-recetario.herokuapp.com/api/plato"),
        description:
          String::from("RESTful API done with Spring Boot 2.4.5, Gradle 6.8.2 and Java JDK 11+. Used MVC, Hibernate/JPA, QueryDSL, HATEOAS, JUnit, Mockito, custom mapper, common interfaces, dockerfile, docker-compose, YAML formatting, etc. MySQL DB is hosted by Heroku's JawsDB service. Coded with VSCode. JawsDB service shutsdown the DB after 30 min of inactivity, so if you don't see any content let it boot."),
      },
      Project{
        id: String::from("AoC2020"),
        pushed_at: String::from(""),
        name: String::from("Advent of Code (2020)"),
        image: String::from("./images/aoc2020.png"),
        repo: String::from("https://github.com/mezdelex/AoC2020"),
        app: String::from("https://adventofcode.com/2020"),
        description:
          String::from("Algorithmic problem solving to achieve further Java's JDK 11+ language proficiency in declarative paradigm using streams, lambdas and functional interfaces. Coded with VSCode."),
      },
      Project{
        id: String::from("HackerRank"),
        pushed_at: String::from(""),
        name: String::from("HackerRank"),
        image: String::from("./images/hackerrank.png"),
        repo: String::from("https://github.com/mezdelex/HackerRank"),
        app: String::from("https://www.hackerrank.com/alexcondegomez"),
        description:
          String::from("HackerRank exercises to keep algorithmic problem solving fresh and improve my expertise with Java's JDK 11+ declarative and imperative syntax and to learn functional programming paradigm using Haskell. Coded with VSCode."),
      },
      Project{
        id: String::from("Trivia"),
        pushed_at: String::from(""),
        name: String::from("Trivia"),
        image: String::from("./images/trivia.jpg"),
        repo: String::from("https://github.com/mezdelex/Trivia"),
        app: String::from("https://mezdelex.github.io/Trivia/trivial_main.html"),
        description:
          String::from("My first Web App back in 2020 done with Javascript, HTML, CSS and Bootstrap consuming an external Trivia API that required token authorization to be accessed. Coded with VSCode."),
      }
]
});
