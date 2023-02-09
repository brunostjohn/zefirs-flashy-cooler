module.exports = {
  packagerConfig: {
    "name": "Zefir's Flashy Cooler",
    "win32metadata": {
    }
  },
  rebuildConfig: {},
  makers: [
    {
      name: '@electron-forge/maker-squirrel',
      config: {
        "name": "CapellixLCD"
      },
    },
  ]
};
