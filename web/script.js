document.getElementById("submit").addEventListener("click", function () {
  location.href = `/write/${document.getElementById("name").value}`;
});
