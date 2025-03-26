const otpInputs = document.querySelectorAll("#otp-container input");

otpInputs.forEach((input, index) => {
  input.addEventListener("input", (e) => {
    const value = e.target.value;
    if (/^\d$/.test(value)) {
      if (index < otpInputs.length - 1) {
        otpInputs[index + 1].focus();
      }
    } else {
      e.target.value = "";
    }
  });

  input.addEventListener("keydown", (e) => {
    if (e.key === "Backspace" && e.target.value === "" && index > 0) {
      otpInputs[index - 1].focus();
    }
  });
});