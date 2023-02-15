const config = {
	content: [
		"./src/**/*.{html,js,svelte,ts}",
		"./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
	],
	
	theme: {
	  extend: {
		fontFamily: {
		  monda: ["Monda", "sans-serif"],
		  roboto: ["Roboto", "sans-serif"],
		  signika: ["Signika", "sans-serif"],
		  "palanquin-dark": ["Palanquin Dark", "sans-serif"],
		},
	  }
	},
	plugins: [
	  require('@tailwindcss/typography'),
	  require('@tailwindcss/forms'),
	  require("flowbite/plugin"),
	  require('flowbite-typography'),
	],
	darkMode: 'class',
};
  
module.exports = config;