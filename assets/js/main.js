document.addEventListener("DOMContentLoaded", function () {
  const forms = document.querySelectorAll("form");
  for (const form of forms) {
    form.addEventListener("submit", async function (e) {
      await jsonForms(e);
    });
  }
});
