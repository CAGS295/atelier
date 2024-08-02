
# --- ------------------------------------------------------------------- --- #
# --- trainer.py -------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

import numpy as np
import torch
from torch import optim
from torch import nn

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def train(self, train, target, epochs, target_len):

    losses = np.full(epochs, np.nan)

    optimizer = optim.Adam(self.parameters(), 
                           lr=0.001,
                           eps=1e-9)

    criterion = nn.MSELoss(reduction='mean')

    for e in range(epochs):

        predicted = torch.zeros(target_len,
                                train.shape[1],
                                train.shape[2])
        optimizer.zero_grad()

        _, enc_state = self.encoder(train)
        dec_in = train[-1, :, :]
        dec_state = enc_state

    for t in range(target_len):
        dec_out, dec_state = self.decoder(dec_in, dec_state)
        predicted[t] = dec_out
        dec_in = dec_out

        loss = criterion(predicted, target)
        loss.backward()
        optimizer.step()
        losses[e] = loss.item()

        if e % 10 == 0:
            print(f"epoch {e}/{epochs}: {round(loss.item(), 4)}")

