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
  const checkedInAt = dayjs(Date.now()).to(attendee.checkedInAt);

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
