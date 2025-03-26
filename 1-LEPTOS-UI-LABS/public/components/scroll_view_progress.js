window.addEventListener("scroll", function() {
  var scrollY = window.pageYOffset || document.documentElement.scrollTop;
  var winHeight = window.innerHeight || document.documentElement.clientHeight;
  var docHeight = document.body.scrollHeight || document.documentElement.scrollHeight;
  var scrollPercent = (scrollY / (docHeight - winHeight)) * 100;
  document.querySelector("#readingProgress").style.width = scrollPercent + "%";
});