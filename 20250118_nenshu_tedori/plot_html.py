import numpy as np
import plotly.graph_objects as go
from plotly import offline


alldata = np.loadtxt("./tedori.csv", skiprows=1, delimiter=",")
nenshu = alldata[:, 0] / 10000
tedori = alldata[:, -2] / 10000
rate = alldata[:, -1]

fig1 = go.Figure()
trace1 = go.Scatter(x=nenshu, y=tedori, mode="lines")
fig1.add_trace(trace1)
fig1.update_layout(title="年収 - 手取り",
                   xaxis_title="年収 (万円)",
                   yaxis_title="手取り (万円)")
offline.plot(fig1, filename="tedori.html")

fig2 = go.Figure()
trace2 = go.Scatter(x=nenshu, y=rate, mode="lines")
fig2.add_trace(trace2)
fig2.update_layout(title="年収 - 手取り率",
                   xaxis_title="年収 (万円)",
                   yaxis_title="手取り率 (%)")
offline.plot(fig2, filename="rate.html")
