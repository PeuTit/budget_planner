# Budget Planner

## Goals

Split a calendar year into 12 "Budget months" with a year as input.

### A calendar year:
- 52 weeks (the fist week can overlap between the years)

### A Budget month:
- Contains at least 4 weeks.
- And at most 5 weeks

### A Budget week:
A week cannot be splited*, this means that a budget month can include day from a different calendar month.
When a week overlap between two months, we apply the following rule to decide which month it will be added to:
- The month contains 4 days of the week.
- The month as no more than 5 weeks already.

(* see special case below)

#### Example:
Year 2024
January -> 31 days
Week 1: 01 to 07
Week 2: 08 to 14
Week 3: 15 to 21
Week 4: 22 to 28

We still have 3 days left in January, applying the rules above we don't met the first criteria (day_left >= 4 == false).
So this week will be attributed to the next month, February.

February -> 29 days
Week 1: 29 to 04
Week 2: 05 to 11
Week 3: 12 to 18
Week 4: 19 to 25

In this case, we have 4 days left in January,
we met the requirement for the first rule (day_left >= 4 == true) and the second one (nbr_of_weeks < 5).

Week 5: 26 to 03

And so on....

### Special case:
- January will always start on the 01.
- December will always end on the 31.

## Input:
I would like the input to be as simple as the year in String or Number.

## Output:
Print each months with its corresponding weeks.

## Pipeline | Process

- Receive a year as user input.
- Create a reference to the first date of the year (January first).
- Create a Week struct.
- Generate a list of all the weeks in a year with their number, start & end date.
- Associate each week with a month
