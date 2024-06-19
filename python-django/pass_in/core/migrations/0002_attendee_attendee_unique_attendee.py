# Generated by Django 5.0.6 on 2024-06-10 02:11

import django.db.models.deletion
from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('core', '0001_initial'),
    ]

    operations = [
        migrations.CreateModel(
            name='Attendee',
            fields=[
                ('id', models.AutoField(primary_key=True, serialize=False)),
                ('name', models.TextField()),
                ('email', models.EmailField(max_length=254)),
                ('created_at', models.DateTimeField(auto_now_add=True)),
                ('event', models.ForeignKey(on_delete=django.db.models.deletion.CASCADE, related_name='attendees', to='core.event')),
            ],
            options={
                'db_table': 'attendees',
            },
        ),
        migrations.AddConstraint(
            model_name='attendee',
            constraint=models.UniqueConstraint(fields=('email', 'event'), name='unique_attendee'),
        ),
    ]
