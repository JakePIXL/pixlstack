<script>
    import { Card } from "flowbite-svelte";
    import Icon from '@iconify/svelte';
</script>

<div class="container mx-auto flex flex-col md:flex-row gap-4">
    <Card size="xl" class="gap-2 h-fit md:w-1/4">
        <img src="/picofme.png" alt="Jake Evans blue background selfie" class="rounded">
        <div class="flex flex-row justify-between items-center">
            <h1 class="font-palanquin-dark text-2xl">
                Jake Evans
            </h1>
            <p class="font-palanquin-dark flex flex-row items-center gap-1 pt-1.5">
                <Icon icon="twemoji:flag-canada" />
                Ontario, Canada
            </p>
        </div>
    </Card>
    <div class="flex flex-col gap-4 w-full md:w-3/4">
        <Card size="xl" class="format min-w-full dark:format-invert font-roboto">
            <h1
            class="h-fit w-fit border-l-2 border-blue-500 pl-2 font-palanquin-dark text-2xl tracking-wide"
            >
            About Me
            </h1>
            <p>
            Hello, I'm Jake, a backend developer and a seasoned budtender located in
            Ontario, Canada. My expertise lies in Python programming, however, I am
            constantly seeking new challenges and opportunities to expand my skill
            set, including Rust and embedded programming.
            <br >
            <br >
            My career goal is to work in the field of backend development and tackle
            complex systems either as an entrepreneur or within the cannabis
            industry. In addition, cybersecurity is a subject that fascinates me and
            I take great pride in using my problem-solving skills to create
            efficient and secure solutions. In my leisure time, I enjoy playing
            video games, staying up-to-date on the latest technology advancements,
            and working on various personal projects that allow me to exercise my
            problem-solving abilities.
            </p>
        </Card>
        <Card size="xl" class="format min-w-full dark:format-invert font-roboto">
            <h1
            class="h-fit w-fit border-l-2 border-blue-500 pl-2 font-palanquin-dark text-2xl tracking-wide"
            >
            Skills & Knowledge
            </h1>
            <div class="">
            <p>I possess a diverse set of expertise in the backend development field and have a proven track record of delivering high-quality results with a focus on performance, scalability, and security. My technical stack encompasses the following:</p>
                <ul class="space-y-2.5 text-gray-500 list-disc list-outside dark:text-gray-400 w-full pr-4">
                    <li class="w-full"><strong>Python</strong>
                    <ul class="space-y-1 text-gray-500 list-disc list-inside dark:text-gray-400">
                        <li><strong>FastAPI</strong> - Utilizing this modern framework to create efficient, scalable APIs with ease.</li>
                        <li><strong>Pandas/Polars</strong> - Experience with the popular data manipulation and analysis libraries and its rust based alternative, Polars, for high-performance data processing.</li>
                        <li><strong>PyTorch</strong> - Knowledge in the use of this deep learning framework for building powerful machine learning models.</li>
                    </ul>
                    </li>
                    <li class="w-full"><strong>Rust</strong>
                    <ul class="space-y-1 text-gray-500 list-disc list-inside dark:text-gray-400">
                        <li><strong>Actix Web</strong> - Proficiency in using this highly performant and modular web framework for building web applications.</li>
                        <li><strong>Tauri</strong> - Proficiency in using this highly performant and secure framework for building cross-platform desktop applications.</li>
                        <li><strong>Polars</strong> - Experience in using this fast and memory efficient data processing library for both Python and Rust.</li>
                    </ul>
                    </li>
                    <li class="w-full"><strong>Kafka</strong> - Knowledge in utilizing this distributed event streaming platform for data processing and analysis.</li>
                    <li class="w-full"><strong>Postgres</strong> - Expertise in utilizing this powerful relational database management system for efficient data storage and retrieval.</li>
                    <li class="w-full"><strong>SQL</strong> - Proficiency in writing efficient and optimized SQL queries for data retrieval and analysis.</li>
                    <li class="w-full"><strong>Redis</strong> - Knowledge in utilizing this in-memory data structure store for efficient data caching and retrieval.</li>
                    <li class="w-full"><strong>Docker</strong> - Expertise in utilizing this containerization platform for efficient application deployment and management.</li>
                    <li class="w-full"><strong>Git</strong> - Proficiency in utilizing this version control system for efficient code management and collaboration.</li>
                    <li class="w-full"><strong>Linux</strong> - Knowledge in using this open-source operating system for efficient server administration and management.</li>
                    <li class="w-full"><strong>WebAuthn</strong> - Familiarity with this web standard for secure and convenient authentication with biometrics and security keys.</li>
                </ul>
            <p>With my extensive knowledge in various technologies and tools, I am confident in my ability to deliver results and exceed expectations in backend development projects. </p>
            </div>
        </Card>
    </div>
</div>