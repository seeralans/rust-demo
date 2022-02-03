import numpy as np
import matplotlib.pyplot as pp
pp.style.use(["default", "paper"])

def coverage_sim(n0, p, nn, num_trials):

  trial_times = np.zeros(num_trials)
  for trial in range(num_trials):
    visits = np.zeros(nn, dtype=int)
    current_pos = n0
    visits[current_pos-1] += 1
    time = 0
    while not visits.all() > 0:
      # draw a random number
      val = np.random.rand()
      # stay put
      if val < 1 - p:
        continue
      # move to the left
      elif val < 1 - p/2:
        current_pos -= 1
      # move to the right
      else:
        current_pos += 1

      # boundary checks
      if current_pos < 1:
        current_pos += 1
      elif current_pos > nn:
        current_pos -= 1
      # increment visit
      visits[current_pos-1] += 1
      # increment time
      time += 1
    trial_times[trial] = time
  return trial_times
