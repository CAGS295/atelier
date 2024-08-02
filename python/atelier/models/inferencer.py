
# --- ------------------------------------------------------------------- --- #
# --- inferencer.py ----------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

import torch

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

def predict(self, x, target_len):

    y = torch.zeros(target_len, x.shape[1], x.shape[2])
    _, enc_state = self.encoder(x)
    dec_in = x[-1, :, :]
    dec_state = enc_state

    for t in range(target_len):
        dec_out, dec_state = self.decoder(dec_in, dec_state)
        y[t] = dec_out
        dec_in = dec_out
    
    return y

