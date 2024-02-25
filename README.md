<!-- <h1 align="center"> -->
<!--     <img src="./assets/logos/Neovim.png" width=256 alt="warden">&#x200B; -->
<!-- </h1> -->
<h2 align="center">
    A self hostable nixpkgs review bot
</h2>

<h1 align="center">
<a href='#'><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="600px"/></a>
  <br>
  <br>
  <div>
    <a href="https://github.com/Iogamaster/warden/issues">
        <img src="https://img.shields.io/github/issues/Iogamaster/warden?color=fab387&labelColor=303446&style=for-the-badge">
    </a>
    <a href="https://github.com/Iogamaster/warden/stargazers">
        <img src="https://img.shields.io/github/stars/Iogamaster/warden?color=ca9ee6&labelColor=303446&style=for-the-badge">
    </a>
    <a href="https://github.com/Iogamaster/warden">
        <img src="https://img.shields.io/github/repo-size/Iogamaster/warden?color=ea999c&labelColor=303446&style=for-the-badge">
    </a>
    <a href="https://github.com/Iogamaster/warden/blob/main/.github/LICENCE">
        <img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=ca9ee6&colorA=313244&colorB=cba6f7"/>
    </a>
    <br>
    </div>
        <img href="https://builtwithnix.org" src="https://builtwithnix.org/badge.svg"/>
   </h1>
   <br>
   
Things warden already does:
- [ ] Build all packages updated/added in a pr (by parsing commits)
- [ ] Run statix and deadnix on all code
- [ ] Run nixpkgs-lint for all packages update/added in a pr (by parsing commits)
- [ ] Summarize the information

Things to do:
- [ ] Make it an actual bot
- [ ] Verify passthru.tests https://github.com/IogaMaster/warden/issues/1
- [ ] Clean up the codebase
- [ ] Allow for reviewing HEAD of the current repo
- [ ] Use the new version of nixpkgs-lint
