module.exports = {
  packagerConfig: {
    name: "Zefir's Flashy Cooler",
    win32metadata: {},
    icon: "./assets/images/favicon.ico",
  },
  rebuildConfig: {},
  makers: [
    {
      name: "@electron-forge/maker-squirrel",
      config: {
        name: "zefirs-flashy-cooler",
        setupIcon: "./assets/images/favicon.ico",
        iconUrl:
          "https://github.com/brunostjohn/zefirs-flashy-cooler/blob/main/assets/images/favicon.ico?raw=true",
        loadingGif: "./assets/images/installer.gif",
      },
    },
  ],
  publishers: [
    {
      name: "@electron-forge/publisher-github",
      config: {
        repository: {
          owner: "brunostjohn",
          name: "zefirs-flashy-cooler",
        },
      },
      prerelease: false,
      draft: true,
      authToken: process.env.GITHUB_TOKEN,
    },
  ],
};
