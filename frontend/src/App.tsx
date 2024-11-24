import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid2';
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});


import KlipperScreenSetting from './components/KlipperScreenSetting';

function App() {

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <AppBar>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            KlipperSetupHelper
          </Typography>
        </Toolbar>
      </AppBar>
      <Box component="main">
        <Toolbar />
        <Grid container spacing={2}       
          sx={{
          marginX: 4, // 水平方向のマージン
          marginY: 2, // 垂直方向のマージン
          }}
        >
          <KlipperScreenSetting></KlipperScreenSetting>
        </Grid>
      </Box>
    </ThemeProvider>
  )
}

export default App
