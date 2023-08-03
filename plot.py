#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Jul 24 20:24:38 2023

@author: mairieliw
"""

import sys
import pandas as pd
import seaborn as sns

[_,csvfile,pngfile,*_] = sys.argv 

df = pd.read_csv(csvfile)

sns.set_theme(style="whitegrid")

g = sns.catplot(
    data=df, kind="bar",
    x="bandit", y="payoff_mean", hue="strategy",
    palette="dark", alpha=.6, height=6
)

g.despine(left=True)
g.savefig(pngfile)