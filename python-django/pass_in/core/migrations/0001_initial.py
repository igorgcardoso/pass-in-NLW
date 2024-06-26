# Generated by Django 5.0.6 on 2024-06-10 01:59

import uuid
from django.db import migrations, models


class Migration(migrations.Migration):

    initial = True

    dependencies = [
    ]

    operations = [
        migrations.CreateModel(
            name='Event',
            fields=[
                ('id', models.CharField(default=uuid.uuid4, max_length=36, primary_key=True, serialize=False)),
                ('title', models.TextField()),
                ('details', models.TextField(blank=True, null=True)),
                ('slug', models.SlugField(max_length=255, unique=True)),
                ('maximum_attendees', models.PositiveIntegerField(null=True)),
            ],
            options={
                'db_table': 'events',
            },
        ),
    ]
