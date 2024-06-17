const attendees = [
  {
    name: "Diego Fernandes",
    email: "diego@gmail.com",
    subscribedAt: new Date(2024, 2, 1, 19, 23),
    checkedInAt: new Date(2024, 2, 1, 20, 20),
  },
  {
    name: "Mayk Brito",
    email: "mayk@gmail.com",
    subscribedAt: new Date(2024, 2, 23, 19, 23),
    checkedInAt: null,
  },
  {
    name: "Ana Souza",
    email: "ana@gmail.com",
    subscribedAt: new Date(2024, 0, 3, 19, 23),
    checkedInAt: new Date(2024, 0, 4, 20, 20),
  },
  {
    name: "João Silva",
    email: "joao@gmail.com",
    subscribedAt: new Date(2023, 11, 4, 19, 23),
    checkedInAt: new Date(2023, 11, 5, 20, 20),
  },
  {
    name: "Maria Oliveira",
    email: "maria@gmail.com",
    subscribedAt: new Date(2023, 10, 5, 19, 23),
    checkedInAt: null,
  },
  {
    name: "Pedro Santos",
    email: "pedro@gmail.com",
    subscribedAt: new Date(2023, 9, 6, 19, 23),
    checkedInAt: new Date(2023, 9, 7, 20, 20),
  },
  {
    name: "Carla Lima",
    email: "carla@gmail.com",
    subscribedAt: new Date(2023, 8, 7, 19, 23),
    checkedInAt: new Date(2023, 8, 8, 20, 20),
  },
  {
    name: "Lucas Sousa",
    email: "lucas@gmail.com",
    subscribedAt: new Date(2023, 7, 8, 19, 23),
    checkedInAt: new Date(2023, 7, 9, 20, 20),
  },
  {
    name: "Paula Costa",
    email: "paula@gmail.com",
    subscribedAt: new Date(2023, 6, 9, 19, 23),
    checkedInAt: null,
  },
  {
    name: "Gabriel Almeida",
    email: "gabriel@gmail.com",
    subscribedAt: new Date(2023, 5, 10, 19, 23),
    checkedInAt: null,
  },
];

function createNewAttendee(attendee) {
  const subscribedAt = dayjs(Date.now()).to(attendee.subscribedAt);
  let checkedInAt = dayjs(Date.now()).to(attendee.checkedInAt);

  if (attendee.checkedInAt == null) {
    checkedInAt = `
    <button data-email="${attendee.email}" onclick="makeCheckIn(event)">
      Confirmar check-in
    </button>
    `;
  }

  return `
  <tr>
    <td>
      <strong>
        ${attendee.name}
      </strong>
      <br />
      <small>
        ${attendee.email}
      </small>
    </td>
    <td>
      ${subscribedAt}
    </td>
    <td>
      ${checkedInAt}
    </td>
  </tr>
  `;
}

function updateList(attendees) {
  let output = "";

  for (let attendee of attendees) {
    output += createNewAttendee(attendee);
  }

  document.querySelector("tbody").innerHTML = output;
}

updateList(attendees);

function addAttendee(event) {
  event.preventDefault();

  const formData = new FormData(event.target);

  const attendee = {
    name: formData.get("name"),
    email: formData.get("email"),
    subscribedAt: new Date(),
    checkedInAt: null,
  };

  if (attendees.some((a) => a.email == attendee.email)) {
    alert("Este e-mail já está cadastrado");
    return;
  }

  attendees.push(attendee);
  updateList(attendees);

  event.target.reset();
}

function makeCheckIn(event) {
  if (!confirm("Tem certeza que deseja fazer check-in?")) {
    return;
  }

  const attendee = attendees.find(
    (attendee) => attendee.email == event.target.dataset.email,
  );

  attendee.checkedInAt = new Date();

  updateList(attendees);
}
