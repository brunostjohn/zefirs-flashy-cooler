module.exports = {
  packagerConfig: {
    "name": "CapellixLCD",
    "win32metadata": {
      "requested-execution-level": "requireAdministrator"
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
