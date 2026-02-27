/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                primary: "#00FF66",
                "background-light": "#FFFFFF",
                "background-dark": "#0A0A0A",
            },
            fontFamily: {
                display: ["Inter", "sans-serif"],
            },
            borderRadius: {
                DEFAULT: "0px",
            },
        },
    },
    plugins: [],
}
