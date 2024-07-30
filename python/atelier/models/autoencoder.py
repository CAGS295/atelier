import torch
import torch.nn as nn
import torch.optim as optim
from torch.nn import functional as F
import numpy as np

# Set seed for reproducibility
seed = 42
np.random.seed(seed)
torch.manual_seed(seed)

device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
dtype = torch.float16

n_past = 30
n_dims = 51
n_future = 10
encoder_units_1 = 200
decoder_units_1 = 200
dropout_value = 0.4

class Encoder(nn.Module):
    
    def __init__(self, n_past, n_dims, encoder_units_1):
        
        super(Encoder, self).__init__()

        self.encoder_inputs = nn.Linear(n_past * n_dims, encoder_units_1)
        self.encoder_l1 = nn.LSTM(input_size=n_dims, 
                                  hidden_size=encoder_units_1, 
                                  num_layers=1, 
                                  batch_first=True)

    def forward(self, inputs):
        
        outputs1 = self.encoder_inputs(inputs.view(-1, n_past * n_dims))
        outputs1, (state_h1, state_c1) = self.encoder_l1(outputs1.unsqueeze(0))
        states1 = [state_h1, state_c1]
        
        return outputs1, states1

class Decoder(nn.Module):
    
    def __init__(self, encoder_units_1, decoder_units_1): 
        super(Decoder, self).__init__()

        self.decoder_state_h = nn.Linear(encoder_units_1, decoder_units_1)
        self.decoder_state_c = nn.Linear(encoder_units_1, decoder_units_1)
        self.decoder_l1 = nn.LSTM(input_size=n_dims,
                                  hidden_size=decoder_units_1,
                                  num_layers=1, 
                                  batch_first=True)

    def forward(self, inputs, initial_state):

        outputs1 = self.decoder_l1(inputs, initial_state=initial_state)

        return outputs1

class OutputLayer(nn.Module):

    def __init__(self, output_dim):
        super(OutputLayer, self).__init__()

        self.output_layer = nn.Linear(output_dim, 10)

    def forward(self, inputs):

        outputs1 = self.output_layer(inputs)

        return outputs1

encoder_inputs = torch.randn((n_past, n_dims), device=device, dtype=dtype)
encoder_l1 = Encoder(n_past, n_dims, encoder_units_1).to(device)
