import { prisma } from "../src/lib/prisma";

async function seed() {
  await prisma.event.create({
    data: {
      id: "4290c3af-7be6-4abf-9f29-172c2b2907ff",
      title: "My First Event",
      details: "This is my first event.",
      slug: "my-first-event",
      maximumAttendees: 128,
    },
  });
}

seed().then(() => {
  console.log("Database seeded!");
  prisma.$disconnect();
});
