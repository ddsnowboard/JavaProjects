#!/usr/bin/python3
# My implementation of Richard Dawkin's weasel program, complete with questionable algorithms

from random import choice, randint, random
# OUTCOME = "METHINKS IT LOOKS LIKE A WEASEL"
# OUTCOME = "LOREM IPSUM DOLOR SIT AMET CONSECTETUR ADIPISCING ELIT PRAESENT PORTA DOLOR SIT AMET BLANDIT IMPERDIET PURUS SEM ACCUMSAN SEM QUIS PLACERAT ELIT ENIM SIT AMET ARCU CUM SOCIIS NATOQUE PENATIBUS ET MAGNIS DIS PARTURIENT MONTES NASCETUR RIDICULUS MUS UT SOLLICITUDIN CONSECTETUR NUNC UT MALESUADA ODIO ALIQUAM UT VESTIBULUM ANTE IPSUM PRIMIS IN FAUCIBUS ORCI LUCTUS ET ULTRICES POSUERE CUBILIA CURAE MAURIS NEC EX NEQUE UT NEC MASSA IN MI IMPERDIET IACULIS AT QUIS NEQUE MAECENAS NON LOREM ET AUGUE BIBENDUM VARIUS VIVAMUS EGET INTERDUM NULLA INTEGER UT DOLOR ORNARE LUCTUS NIBH A FACILISIS ANTE CRAS CURSUS SUSCIPIT NEQUE QUIS DAPIBUS LOREM VESTIBULUM ELEIFEND VESTIBULUM VEL ARCU ERAT IN FEUGIAT UT TURPIS VITAE PRETIUM VESTIBULUM NISL LIGULA ULTRICIES AT MOLESTIE EGET SODALES NON VELIT PHASELLUS AC ELIT SIT AMET NISI VARIUS PLACERAT NEC EU NISI PRAESENT FINIBUS MATTIS URNA UT PORTTITOR LEO RHONCUS VEL SUSPENDISSE VOLUTPAT EGESTAS SODALES FUSCE AC JUSTO UT MI POSUERE"
OUTCOME = "                                                                                          "
print("OUTCOME is {} letters long".format(len(OUTCOME)))
LETTERS = "ABCDEFGHIJKLMNOPQURSTUVWXYZ "
MIN_OFFSPRING = 1
MAX_OFFSPRING = 1000

def replicate(s):
    CHANCE_OF_MUTATION = .05
    out = ""
    for i in s:
        if random() < CHANCE_OF_MUTATION:
            out += choice(LETTERS)
        else:
            out += i
    return out

def distance(s1, s2):
    if len(s1) != len(s2):
        raise Exception("Those strings aren't equal!")
    else:
        out = 0
        for i in range(len(s1)):
            if s1[i] != s2[i]:
                out += 1
        return out
s = ''
nextGen = []
generations = 0
for i in range(len(OUTCOME)):
    s += choice(LETTERS)
while s != OUTCOME:
    nextGen = []
    for i in range(randint(MIN_OFFSPRING, MAX_OFFSPRING)):
        nextGen.append(replicate(s))
    s = min(nextGen, key=lambda x: distance(x, OUTCOME))
    print(s)
    print("\n")
    generations += 1
print("It took {} generations".format(generations))
