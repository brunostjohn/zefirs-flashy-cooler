<a name="readme-top"></a>



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler">
    <img src="static/images/android-chrome-192x192.png" alt="Logo" width="150" height="150">
  </a>

<h1 align="center">Zefir's Flashy Cooler</h1>

  <p align="center">
    Elevate your cooler with modern and reactive themes.
    <br />
    <a href="https://zefirsflashycooler.app"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/releases/">Download now</a>
    ·
    <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/issues">Report Bug</a>
    ·
    <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/issues">Request Feature</a>
  </p>
  <p align="center">
    <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/brunostjohn/zefirs-flashy-cooler" alt="contributors" />
  </a>
  <a href="">
    <img src="https://img.shields.io/github/last-commit/brunostjohn/zefirs-flashy-cooler" alt="last update" />
  </a>
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/network/members">
    <img src="https://img.shields.io/github/forks/brunostjohn/zefirs-flashy-cooler" alt="forks" />
  </a>
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/stargazers">
    <img src="https://img.shields.io/github/stars/brunostjohn/zefirs-flashy-cooler" alt="stars" />
  </a>
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/issues/">
    <img src="https://img.shields.io/github/issues/brunostjohn/zefirs-flashy-cooler" alt="open issues" />
  </a>
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/brunostjohn/zefirs-flashy-cooler.svg" alt="license" />
  </a>
  <a href="https://github.com/brunostjohn/zefirs-flashy-cooler/releases/">
    <img src="https://img.shields.io/github/downloads/brunostjohn/zefirs-flashy-cooler/total" />
  </a>
  <a href="https://linkedin.com/in/brunostjohn">
    <img src="https://img.shields.io/badge/-LinkedIn-black.svg?logo=linkedin&colorB=555" />
  </a>
  </p>
</div>

## 💖 Sponsors

Thank you for your ongoing support. [Click here to become my sponsor.](https://github.com/sponsors/brunostjohn)

<!-- sponsors --><!-- sponsors -->

<!-- TABLE OF CONTENTS -->
<details>
  <summary>📔Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## 🌟 About The Project

[![Product Name Screen Shot][product-screenshot]](https://zefirsflashycooler.app)

This project aims to entirely replace iCUE and other pieces of software like it in control of CPU coolers etc. with LCD displays.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### 👾 Built With

* ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
* ![OpenGL](https://img.shields.io/badge/OpenGL-%23FFFFFF.svg?style=for-the-badge&logo=opengl)
* [![Svelte][Svelte.dev]][Svelte-url]
* ![C#](https://img.shields.io/badge/C%23-239120?style=for-the-badge&logo=c-sharp&logoColor=white)
* ![C](https://img.shields.io/badge/C-00599C?style=for-the-badge&logo=c&logoColor=white)
* ![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
* ![SASS](https://img.shields.io/badge/SASS-hotpink.svg?style=for-the-badge&logo=SASS&logoColor=white)
* [![Bootstrap][Bootstrap.com]][Bootstrap-url]
* ![Vite](https://img.shields.io/badge/vite-%23646CFF.svg?style=for-the-badge&logo=vite&logoColor=white)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## 🧰 Getting Started

To build this project locally, you will need a few dependencies set up.

### ‼️ Prerequisites

1. Set up a Rust toolchain using [Rustup](https://rustup.rs/).
2. Install cargo-make by running `cargo install cargo-make`.
3. Install Visual Studio Community 2022 with a C# workload.
4. Install Chocolatey.
5. Install the following dependencies from Chocolatey:
   - LLVM 15.0.7
   - NASM
   - CMake
6. Run `cargo make` in `src-tauri`. The project should build normally.

If `System.IO.Compression.Native.Aot.lib` throws linker conflicts for you, run the following command from a Visual Studio Developer Command Prompt:

```lib /remove:libs-native\System.IO.Compression.Native\CMakeFiles\System.IO.Compression.Native.Aot.dir\D\_\a_work\1\s\src\native\external\brotli\dec\decode.c.obj /LIBPATH:[WHEREEVER YOU HAVE THE REPO ON YOUR MACHINE]zefirs-flashy-cooler\src-tauri\static-libs .\System.IO.Compression.Native.Aot.lib```

These symbols are still covered by a different library and everything still functions correctly.

In case of further issues, follow the error messages until everything works. For support, try messaging me through [our discord server](https://discord.gg/KXmnwA6zWK).
### ⚙️ Installation

1. Download the [latest app release from GitHub](https://github.com/brunostjohn/zefirs-flashy-cooler/releases/).
2. Follow the installer.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## 🏃 Usage

To use this app without making themes, just download some from the Theme Store and enjoy. It should work right out of the box.

_For more information, please refer to the [Documentation](https://zefirsflashycooler.app)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## 🎯 Roadmap

- [ ] A complete rewrite in Rust.
- [ ] Support for more coolers (including NZXT).
- [ ] More app stability.
- [ ] More services for theme creators.
- [ ] Better performance metrics.
- [ ] Linux support.

<!-- See the [open issues](https://github.com/brunostjohn/zefirs-flashy-cooler/issues) for a full list of proposed features (and known issues). -->

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## 👋 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the project.
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## ⚠️ License

Distributed under the AGPLv3 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## 🤝 Contact

Bruno St. John - me@brunostjohn.com

Project Link: [https://github.com/brunostjohn/zefirs-flashy-cooler](https://github.com/brunostjohn/zefirs-flashy-cooler)

Website: [https://zefirsflashycooler.app](https://zefirsflashycooler.app)

Discord: [https://discord.gg/KXmnwA6zWK](https://discord.gg/KXmnwA6zWK)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## 💎 Acknowledgments

* Zefir for being the best cat on Planet Earth.
* Klaudia for living with me as I am visibly frustrated at a computer.
* The King Boardstar for being willing to test software that doesn't even work on his hardware.
* Dordo for bootstrapping this project by helping with reverse engineering the Capellix LCD.
* Heal-bot for yelling at me when I do stupid stuff.
* Joe.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


[contributors-shield]: https://img.shields.io/github/contributors/brunostjohn/zefirs-flashy-cooler.svg?style=for-the-badge
[gh_dls]: https://img.shields.io/github/downloads/brunostjohn/zefirs-flashy-cooler/total?style=for-the-badge
[contributors-url]: https://github.com/brunostjohn/zefirs-flashy-cooler/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/brunostjohn/zefirs-flashy-cooler.svg?style=for-the-badge
[forks-url]: https://github.com/brunostjohn/zefirs-flashy-cooler/network/members
[stars-shield]: https://img.shields.io/github/stars/brunostjohn/zefirs-flashy-cooler.svg?style=for-the-badge
[stars-url]: https://github.com/brunostjohn/zefirs-flashy-cooler/stargazers
[issues-shield]: https://img.shields.io/github/issues/brunostjohn/zefirs-flashy-cooler.svg?style=for-the-badge
[issues-url]: https://github.com/brunostjohn/zefirs-flashy-cooler/issues
[license-shield]: https://img.shields.io/github/license/brunostjohn/zefirs-flashy-cooler.svg?style=for-the-badge
[license-url]: https://github.com/brunostjohn/zefirs-flashy-cooler/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/brunostjohn
[product-screenshot]: static/images/screenshot.png
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com