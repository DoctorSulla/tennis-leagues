async function jsonForms(e) {
  console.log(e);
  e.preventDefault();
  const action = e.target.action;
  const method = e.target.dataset.method.toUpperCase();
  const payload = {};
  const fields = e.target.querySelectorAll("input");
  for (const field of fields) {
    if (field.type == "number") {
      payload[field.name] = parseInt(field.value);
    } else {
      payload[field.name] = field.value;
    }
  }
  const options = {};
  options.redirect;
  options.body = JSON.stringify(payload);
  options.method = method;
  options.headers = {
    "Content-Type": "application/json",
  };
  try {
    const response = await fetch(action, options);
    handleResponse(response);
  } catch (e) {
    console.error(e);
  }
}

async function handleResponse(response) {
  console.log(response.status);
  if (response.headers.get("Location")) {
    location.assign(response.headers.get("Location"));
  } else if (response.status == 205) {
    window.location.reload();
  } else if (response.ok) {
    const json = await response.json();
    // switch (json.type) {
    //   case
    // }
  }
}
