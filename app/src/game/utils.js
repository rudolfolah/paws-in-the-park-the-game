export function shuffle(items) {
  const shuffled = [...items];
  for (let i = 0; i < shuffled.length; i += 1) {
    const j = Math.floor(Math.random() * shuffled.length);
    const temp = shuffled[i];
    shuffled[i] = shuffled[j];
    shuffled[j] = temp;
  }
  return shuffled;
}
