# Adaptive Elo Game

The goal of this project is to simulate and with that try out different systems for adaptively adjusting students and questions according to [IRT](https://en.wikipedia.org/wiki/Item_response_theory) (Item Response Theory) by using an [Elo-System](https://en.wikipedia.org/wiki/Elo_rating_system) as a basis and making modifications to it based on certain assumptions.

## Elo for adaptive learning
The idea is that you can interprete a student taking a question as two contestants in a competitive game of, for example, chess. The student and the question each representing one player.

If the student answers the question correctly it is counted as a *win* for the student, if they answer it wrongly it counts as a *lose* for the student and therefore as a *win* for the question. The ratings of both contestants would then be updated by a certain amount. That update depends on how *likely* or *surprising* the result was.

Both contestants having the exact same Elo rating in this context meaning they have equal chances of winning this match, 50% each. A lower rated contestant would therefore have a lower chance of winning against a higher rated one. Different Elo systems use different ways for calculating the chances of winning (expected score) but they usually involve some kind of [logistic](https://en.wikipedia.org/wiki/Logistic_function) or more generally [sigmoid function](https://en.wikipedia.org/wiki/Sigmoid_function).

## Asymmetry
The original El rating assumes 
* Both, students and questions, get a integer number to respresent their *strength* and *difficulty* respectively.
* A student having the same rating as the question should represent a 99% chance of the student answering that question correctly.
* A student having half the rating of a question should represent a 50% chance of the student answering that question correctly.
