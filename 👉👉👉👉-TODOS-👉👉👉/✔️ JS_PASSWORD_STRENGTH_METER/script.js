document.addEventListener("DOMContentLoaded", () => {
    const passwordInput = document.getElementById("password");
    const togglePassword = document.getElementById("toggle-password");
    const strengthFill = document.getElementById("strength-fill");
    const strengthText = document.getElementById("strength-text");
    const lengthReq = document.getElementById("length");
    const uppercaseReq = document.getElementById("uppercase");
    const lowercaseReq = document.getElementById("lowercase");
    const numberReq = document.getElementById("number");
    const specialReq = document.getElementById("special");
  
    // Toggle password visibility
    togglePassword.addEventListener("click", () => {
      const type = passwordInput.getAttribute("type") === "password" ? "text" : "password";
      passwordInput.setAttribute("type", type);
  
      // Toggle eye icon
      const icon = togglePassword.querySelector("i");
      icon.classList.toggle("fa-eye");
      icon.classList.toggle("fa-eye-slash");
    });
  
    // Check password strength on input
    passwordInput.addEventListener("input", () => {
      const password = passwordInput.value;
      const container = passwordInput.closest(".password__strength__container");
  
      // Remove all strength classes
      container.classList.remove(
        "strength__very__weak",
        "strength__weak",
        "strength__medium",
        "strength__strong",
        "strength__very__strong",
      );
  
      // Check requirements
      const hasLength = password.length >= 8;
      const hasUppercase = /[A-Z]/.test(password);
      const hasLowercase = /[a-z]/.test(password);
      const hasNumber = /[0-9]/.test(password);
      const hasSpecial = /[^A-Za-z0-9]/.test(password);
  
      // Update requirement status
      updateRequirement(lengthReq, hasLength);
      updateRequirement(uppercaseReq, hasUppercase);
      updateRequirement(lowercaseReq, hasLowercase);
      updateRequirement(numberReq, hasNumber);
      updateRequirement(specialReq, hasSpecial);
  
      // Calculate strength score (0-5)
      let strengthScore = 0;
      if (password.length > 0) strengthScore += 1;
      if (hasLength) strengthScore += 1;
      if (hasUppercase) strengthScore += 1;
      if (hasLowercase) strengthScore += 1;
      if (hasNumber) strengthScore += 1;
      if (hasSpecial) strengthScore += 1;
  
      // Adjust score to max of 5
      strengthScore = Math.min(5, Math.floor((strengthScore * 5) / 6));
  
      // Update strength meter and text
      updateStrengthMeter(container, strengthScore, strengthText);
    });
  
    function updateRequirement(element, isValid) {
      const icon = element.querySelector("i");
  
      if (isValid) {
        element.classList.add("valid");
        icon.classList.remove("fa-times-circle");
        icon.classList.add("fa-check-circle");
      } else {
        element.classList.remove("valid");
        icon.classList.remove("fa-check-circle");
        icon.classList.add("fa-times-circle");
      }
    }
  
    function updateStrengthMeter(container, score, textElement) {
      const strengthClasses = [
        "",
        "strength__very__weak",
        "strength__weak",
        "strength__medium",
        "strength__strong",
        "strength__very__strong",
      ];
  
      const strengthTexts = [
        "Enter a password",
        "Very weak",
        "Weak",
        "Medium",
        "Strong",
        "Very strong",
      ];
  
      // Add appropriate class
      if (score > 0) {
        container.classList.add(strengthClasses[score]);
      }
  
      // Update text
      textElement.textContent = strengthTexts[score];
    }
  });
  