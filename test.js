const dict = require('./dict');
const alphabet = 'abcdefghijklmnopqrstuvwxyz';

//var letters = alphabet.split('');

var pos = [{}, {}, {}, {}, {}];
var letters = {};
var total = 0;

dict.answers.forEach(function(answer) {
	var l = answer.split('');
	total += 5;
	for (b in l) {
		x = l[b];
		if (!letters[x]) letters[x] = 0;
		if (!pos[b][x]) pos[b][x] = 0;
		letters[x] += 1;
		pos[b][x] += 1;
	}
});

/*
console.log('pos,letter,occurrances,percentage');
for (p in pos) {
	var lett = pos[p];
	for (l in lett) {
		console.log(`${parseInt(p)+1},${l},${lett[l]},${lett[l]/(total/5)}`);
	}
}

console.log('letter,occurrances,percentage');
for (l in letters) {
	console.log(`${l},${letters[l]},${letters[l]/total}`)
}
*/
var likely = {};
dict.dict.forEach(function(word) {
	var l = word.split('');
	var prob = 1;
	for (b in l) {
		x = l[b];
		prob *= pos[b][x]/(total/5);
	}
	likely[word] = prob*10000000;
});

console.log('word,percentage');
for (word in likely) {
	console.log(`${word},${likely[word]}`);
}








