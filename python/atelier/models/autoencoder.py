
# --- ------------------------------------------------------------------- --- #
# --- autoencoder.py ---------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

import torch.nn as nn

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

class Encoder(nn.Module):
    
    def __init__(self, input_size, hidden_size, num_layers=1):
        
        super(Encoder, self).__init__()

        self.input_size = input_size
        self.hidden_size = hidden_size
        self.num_layers = num_layers

        self.encoder_l1 = nn.LSTM(input_size=input_size, 
                                  hidden_size=hidden_size,
                                  num_layers=num_layers, 
                                  bias=True,
                                  batch_first=True,
                                  dropout=0,
                                  bidirectional=False,
                                  proj_size=0,)

    def forward(self,
                inputs):
        """

        """
        
        flat_input = inputs.view(inputs.shape[0],
                                 inputs.shape[1],
                                 self.input_size)

        outputs, states = self.encoder_l1(flat_input)

        return outputs, states

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

class Decoder(nn.Module):
    
    def __init__(self, input_size, hidden_size, output_size=1, num_layers=1): 
        super(Decoder, self).__init__()

        self.input_size = input_size
        self.hidden_size = hidden_size
        self.num_layers = num_layers
        self.output_size = output_size
        
        self.decoder_l1 = nn.LSTM(input_size=input_size,
                                  hidden_size=hidden_size,
                                  num_layers=num_layers,
                                  bias=True,
                                  batch_first=True,
                                  dropout=0,
                                  bidirectional=False,
                                  proj_size=0,)

        self.linear = nn.Linear(hidden_size, output_size)

    def forward(self, inputs, states):
        
        outputs, states = self.decoder_l1(inputs.unsqueeze(0), states)
        flat_outputs = self.linear(outputs.squeeze(0))
    
        return flat_outputs, states

# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #
# --- ------------------------------------------------------------------- --- #

class Autoencoder(nn.Module):

    def __init__(self, input_size=1, hidden_size=1, output_size=1):
        super(Autoencoder, self).__init__()

        self.input_size = input_size
        self.output_size = output_size
        self.hidden_size = hidden_size
        
        self.encoder = Encoder(input_size=self.input_size, 
                               hidden_size=self.hidden_size)
        
        self.decoder = Decoder(input_size=self.input_size, 
                               hidden_size=self.hidden_size,
                               output_size=self.output_size)

