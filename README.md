# image-web-server

[AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui): Images in Web Server. Display images in your browser in local network.

# Why / What's this for?

Have you ever want to access ***ONLY*** your outputs images from your phone?   
- Run [AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui) (and install [images-browser](https://github.com/yfszzx/stable-diffusion-webui-images-browser)) **for this**? - [OVERKILL]
- Upload images to cloud service (`Google Drive/Photos` and so on) which have limitation **for that**? - [NUMBERS LIMITATION]
- Transfer files to your phone **for these**? - [PHONE STORAGE / TIME TO TRANSFER FILE]
- What else have we got **for those**? - [IDK]

# Usage

1. ***CHOOSE ONE***
    1. Drop the executable in your [AUTOMATIC1111's Web UI](https://github.com/AUTOMATIC1111/stable-diffusion-webui) root folder (where `webui-user`/`launch.py` is located)
    1. Run executable (anywhere). Don't panic as 'nothing happening' is intended.  
    Now you'll see `iws.config.json` file (contains your config) and `iws.log` folder (contains program log) besides where executable is located.   
    
        Now open `iws.config.json` and edit `images_folder` to absolute path  
        (e.g. `C:/Program Files/stable-diffusion-webui/outputs/txt2img-images`)
        
        > ![image](https://user-images.githubusercontent.com/76484203/219962982-47206d74-0fb6-41e9-a93c-29b40eb350ac.png)
2. Run the executable and you should see something like this
> if nothing is happening (common case)
>  - you follow the second method and did not edit `images_folder` correctly (***maybe you forgor to turn `\` to `/`?***)
>  - Port `80` is binding to something else

![image](https://user-images.githubusercontent.com/76484203/219962289-d39d9d9a-5efe-4a23-bf0b-e86e8992d181.png)

3. Open your favorite browser in your computer and enter `127.0.0.1` or `localhost` and you should see your ever first generated image xD

4. To access from your local network (i.e. your phone with the same network connection), you'll need to know the `Private Address` of your PC.

    - On Windows, you can check by right clicking the taskbar > Task Manager > Performance tab > Ethernet > IPv4 address
      ![image](https://user-images.githubusercontent.com/76484203/219963235-1152b102-b2b9-4985-ae83-69e2c2d161ae.png)

5. Open your favorite browser in your phone and enter the `IPv4 address` we got from the previous step to the address bar

# Default Value

```javascript
{
    images_folder: "./outputs/txt2img-images", // The folder where the images located
    images_per_page: 100,                      // How many images to display in a single page
    html_file: "format.html",                  // Template for website which "{{ data }}" will be replace with images src in that page
}
```

# TODO

- Customizable address (Currently fixed to `0.0.0.0:80`)

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
