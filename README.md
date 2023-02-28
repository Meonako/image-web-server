# image-web-server

[AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui): Images in Web Server. Display images in your browser in local network.

# Why / What's this for?

Have you ever want to access ***ONLY*** your outputs images from your phone?   
- Run [AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui) (and install [images-browser](https://github.com/yfszzx/stable-diffusion-webui-images-browser)) **for this**? - **[OVERKILL]**
- Upload images to cloud service (`Google Drive/Photos` and so on) which have limitation **for that**? - **[CLOUDS STORAGE]**
- Transfer files to your phone **for these**? - **[PHONE STORAGE / TIME TO TRANSFER FILE]**
- What else have we got **for those**? - **[IDK]**

# Get it up and running

1. ***CHOOSE ONE***
    1. Drop the executable in your [AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui) root folder (where `webui-user`/`launch.py` is located)
    1. Run executable (anywhere). Don't panic as 'nothing happening' is intended.  
    Now you'll see `iws.config.json` file (contains your configuration) and `iws.log` folder (contains logs of the executable) besides where executable is located.   
    
        Now open `iws.config.json` and edit the path to absolute path  
        (e.g. `C:/Program Files/stable-diffusion-webui/outputs/txt2img-images`)
        
        > ![image](https://user-images.githubusercontent.com/76484203/221943215-9eb2302e-5192-4a4b-af45-dc64b51f8e37.png)

2. Run the executable and you should see something like this
> if nothing is happening (common case)
>  - you follow the second method and did not edit the path correctly (***maybe you forgor to turn `\` to `/` or `\\`?***)
>  - Port `80` is binding to something else
>  - You do not have a permission to bind port `80`

![image](https://user-images.githubusercontent.com/76484203/221943564-897e9965-0549-47ed-b349-44698d1a9612.png)

3. Open your favorite browser in your computer and enter `127.0.0.1` or `localhost` to the address bar and you should see choice to folders based on what you enable  
![image](https://user-images.githubusercontent.com/76484203/221944419-7c852d8b-e701-4cab-931f-21e18089c0b8.png)

4. To access from your local network (i.e. your phone with the same network connection), you'll need to know the `Private Address` of your PC.

    - On Windows, you can check by right clicking the taskbar > Task Manager > Performance tab > Ethernet > IPv4 address
      ![image](https://user-images.githubusercontent.com/76484203/219963235-1152b102-b2b9-4985-ae83-69e2c2d161ae.png)

5. Open your favorite browser in your phone and enter the `IPv4 address` we got from the previous step to the address bar

# Default Value

```javascript
{
    // Address to bind. 0.0.0.0 will bind every address possible while 80 is the port. So you can access with `localhost` or `127.0.0.1` or your local IPv4
    address: "0.0.0.0:80".to_owned(), 
    
    // Enable Txt2Img folders and where the folder is located. If all folders are disable, it'll panic
    txt2img: Txt2Img {
        enable: true,
        path: "./outputs/txt2img-images".to_owned(),
    },
    
    // Enable Txt2Img Grids folders and where the folder is located. If all folders are disable, it'll panic
    txt2img_grid: Txt2ImgGrid {
        enable: false,
        path: "./outputs/txt2img-grids".to_owned(),
    },
    
    // Enable Img2Img folders and where the folder is located. If all folders are disable, it'll panic
    img2img: Img2Img {
        enable: true,
        path: "./outputs/img2img-images".to_owned(),
    },
    
    // Enable Img2Img Grids folders and where the folder is located. If all folders are disable, it'll panic
    img2img_grid: Img2ImgGrid {
        enable: false,
        path: "./outputs/img2img-grids".to_owned(),
    },
    
    // Enable Extras folders and where the folder is located. If all folders are disable, it'll panic
    extras: Extras {
        enable: true,
        path: "./outputs/extras-images".to_owned(),
    },
    
    // How many images to show in 1 page
    images_per_page: 100,
    
    // Landing page (index page)
    index_file: "index.html".to_owned(),
    
    // HTML file that will show the picture by replace `{{ data }}` with image source
    html_file: "format.html".to_owned(),
}
```

# index.html

```html
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

        img {
            max-width: 100%;
            max-height: 100%;
            margin: 0 auto;
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
```

# format.html

```html
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
```

# Credits

- [actix-web](https://github.com/actix/actix-web) - This project is licensed under either of the following licenses, at your option:
  - Apache License, Version 2.0
  - MIT license
- [actix-files](https://github.com/actix/actix-web/tree/master/actix-files) - This project is licensed under either of the following licenses, at your option:
  - Apache License, Version 2.0
  - MIT license
- [colored](https://github.com/mackwic/colored) - Mozilla Public License 2.0
- [flexi_logger](https://github.com/emabee/flexi_logger) - Found 2 licenses
  - Apache License, Version 2.0
  - MIT License
- [log](https://github.com/rust-lang/log) - Found 2 licenses
  - Apache License, Version 2.0
  - MIT License
- [serde](https://github.com/serde-rs/serde) - Licensed under either of Apache License, Version 2.0 or MIT license at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
  - Apache License, Version 2.0
  - MIT License
- [serde_json](https://github.com/serde-rs/json) - Found 2 licenses
  - Apache License, Version 2.0
  - MIT License
- [regex](https://github.com/rust-lang/regex) - This project is licensed under either of
  - Apache License, Version 2.0
  - MIT license  

    at your option.

    The data in `regex-syntax/src/unicode_tables/` is licensed under the Unicode License Agreement.
- [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) - Licensed under either of
  - Apache License, Version 2.0
  - MIT license  

    at your option.
