function updateClock() {
  const timeInput = document.getElementById('timeInput').value;
  const [hours, minutes] = timeInput.split(':').map(Number);

  const hourAngle = (hours % 12) * 30 + minutes * 0.5;
  const minuteAngle = minutes * 6;

  const hourHand = document.getElementById('hourHand');
  const minHand = document.getElementById('minHand');

  hourHand.setAttribute('transform', `rotate(${hourAngle}, 7, 7)`);
  minHand.setAttribute('transform', `rotate(${minuteAngle}, 7, 7)`);
}

document.getElementById('timeInput').addEventListener('input', updateClock);

updateClock();