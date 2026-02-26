/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./frontend/index.html", "./frontend/src/**/*.rs"],
  safelist: [
    {
      pattern:
        /(text|bg|border)-(red|amber|yellow|emerald|teal|gray|slate|blue)-(50|100|200|300|400|500|600|700|800|900)/,
    },
    {
      pattern: /(from|via|to)-(emerald|teal|amber|orange|gray)-(50|100|200|300|400|500|600|700|800|900)/,
    },
    { pattern: /(w|h)-(1|2|3|4|5|6|7|8|9|10|12|14|16|20|24|32|45|70|80|95)/ },
  ],
  theme: {
    extend: {
      colors: {
        "white-80": "rgba(255, 255, 255, 0.8)",
        "white-95": "rgba(255, 255, 255, 0.95)",
        "white-20": "rgba(255, 255, 255, 0.2)",
      },
      width: {
        45: "45%",
        70: "70%",
        80: "80%",
        95: "95%",
      },
      animation: {
        float: "float 3s ease-in-out infinite",
        "bounce-slow": "bounce-slow 2s ease-in-out infinite",
        "pulse-gentle": "pulse-gentle 2s ease-in-out infinite",
      },
      keyframes: {
        float: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-4px)" },
        },
        "bounce-slow": {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-4px)" },
        },
        "pulse-gentle": {
          "0%, 100%": { opacity: "0.3" },
          "50%": { opacity: "0.5" },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/line-clamp")],
};
