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
![image](https://user-images.githubusercontent.com/76484203/221947675-f9ccefa6-b859-4e25-80be-8eae02c40d3e.png)
    > if nothing is happening (common case)
    >  - you follow the second method and did not edit the path correctly (***maybe you forgor to turn `\` to `/` or `\\`?***)
    >  - Port `80` is binding to something else
    >  - You do not have a permission to bind port `80`

3. Open your favorite browser in your computer and enter `127.0.0.1` or `localhost` to the address bar and you should see choices to each page based on what you enable  
![image](https://user-images.githubusercontent.com/76484203/221944419-7c852d8b-e701-4cab-931f-21e18089c0b8.png)

4. To access from your local network (i.e. your phone with the same network connection), you'll need to know the `Private Address` of your PC.

    - On Windows, you can check by right clicking the taskbar > Task Manager > Performance tab > Ethernet > IPv4 address
      ![image](https://user-images.githubusercontent.com/76484203/219963235-1152b102-b2b9-4985-ae83-69e2c2d161ae.png)

5. Open your favorite browser in your phone and enter the `IPv4 address` we got from the previous step to the address bar

# Default Value

```javascript
{
    // Address to bind. 0.0.0.0 will bind every address possible while 80 is the port. So you can access with `localhost` or `127.0.0.1` or your local IPv4
    address: "0.0.0.0:80", 
    
    // Enable Txt2Img folders and where the folder is located. If all folders are disable, it'll panic
    txt2img: {
        enable: true,
        path: "./outputs/txt2img-images",
    },
    
    // Enable Txt2Img Grids folders and where the folder is located. If all folders are disable, it'll panic
    txt2img_grid: {
        enable: false,
        path: "./outputs/txt2img-grids",
    },
    
    // Enable Img2Img folders and where the folder is located. If all folders are disable, it'll panic
    img2img: {
        enable: true,
        path: "./outputs/img2img-images",
    },
    
    // Enable Img2Img Grids folders and where the folder is located. If all folders are disable, it'll panic
    img2img_grid: {
        enable: false,
        path: "./outputs/img2img-grids",
    },
    
    // Enable Extras folders and where the folder is located. If all folders are disable, it'll panic
    extras: {
        enable: true,
        path: "./outputs/extras-images",
    },
    
    // How many images to show in 1 page
    images_per_page: 100,
    
    // Landing page (index page)
    index_file: "index.html",
    
    // HTML file that will show the picture by replace `{{ data }}` with image source
    html_file: "format.html",
}
```

# Template

MOVED [HERE](https://github.com/Meonako/image-web-server/wiki/Template)

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
