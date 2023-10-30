# Adaptive Elo Game

The goal of this project is to simulate and with that try out different systems for adaptively adjusting students and questions according to [IRT](https://en.wikipedia.org/wiki/Item_response_theory) (Item Response Theory) by using an [Elo-System](https://en.wikipedia.org/wiki/Elo_rating_system) as a basis and making modifications to it based on certain assumptions.

## Elo for adaptive learning

The idea is that you can interprete a student taking a question as two contestants in a competitive game of, for example, chess. The student and the question each representing one player.

If the student answers the question correctly it is counted as a *win* for the student, if they answer it wrongly it counts as a *lose* for the student and therefore as a *win* for the question. The ratings of both contestants would then be updated by a certain amount. That update depends on how *likely* or *surprising* the result was.

Both contestants having the exact same Elo rating in this context meaning they have equal chances of winning this match, 50% each. A lower rated contestant would therefore have a lower chance of winning against a higher rated one. Different Elo systems use different ways for calculating the chances of winning (expected score) but they usually involve some kind of [logistic](https://en.wikipedia.org/wiki/Logistic_function) or more generally [sigmoid function](https://en.wikipedia.org/wiki/Sigmoid_function).

## Disadvantages of the Elo rating

### Asymmetry

The traditional Elo rating suggests a symmetrical view on both contestants. Contestant A playing against contestant B is the same as B playing against A. This doesn't hold true in a learning situation, the student answering the question is not the same as the question answering the student, in fact that doesn't make any sense at all.

## Assumptions

* Students and questions each hold an integer number to respresent their *strength* and *difficulty* respectively.
* A student having the same rating as a question should represent a 99% chance of the student answering that question correctly.
* A student having half the rating of a question should represent a 50% chance of the student answering that question correctly.

* Students who answered a lot of questions within the system have a higher impact on adjusting question ratings.
* Questions that have been answered a lot will be affected less by students answering them.

* The rating update doesn't have to be a [zero-sum game](https://en.wikipedia.org/wiki/Zero-sum_game). If a student gains rating for answering a question correctly, the question doesn't have to lose the same amount of rating.
