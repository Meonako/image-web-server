pub const INDEX_DEFAULT: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index | Images Server</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            background-color: black;
            height: 100vh;
        }

        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100%;
        }

        button {
            font-size: large;
            background-color: inherit;
            border: 1px solid white;
            color: white;
            padding: 1rem 2rem;
            transition: .5s;
            margin-top: 1rem;
            width: 50%;
        }

        button:hover {
            background-color: rgb(119, 119, 119);
            color: black;
        }

        @media (max-width: 500px) {
            button {
                width: 100%;
            }
        }
    </style>
</head>
<body>
    {{ data }}

    <script>
        function handle(btn) {
            window.location.href = `/${btn.id}/1`
        }
    </script>
</body>
</html>
"##;

pub const HTML_DEFAULT: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Images</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            background-color: black;
        }

        img {
            max-width: 100%;
            max-height: 100%;
            margin: 0 auto;
        }

        .nav {
            position: sticky;
            top: 0;
            z-index: 1;
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;
        }

        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
    </style>
</head>
<body>
    <div class="nav">
        <a href="{{ next page }}">Next Page</a>
        <a href="{{ previous page }}">Previous Page</a>
        <a href="{{ last page }}">Last Page</a>
        <button id="reload-btn">Reload</button>
    </div>

    {{ data }}

    <script>
        function main() {
            const reloadBtn = document.querySelector("#reload-btn");
            if (!reloadBtn) return;

            reloadBtn.addEventListener('click', async () => {
                const response = await fetch('{{ reload path }}');
                if (!response.ok) return;

                location.reload();
            })
        }

        main()
    </script>
</body>
</html>
"##;