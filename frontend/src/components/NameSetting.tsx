import Grid from '@mui/material/Grid2';
import Card from '@mui/material/Card'
import { Typography } from '@mui/material';
import CardContent from '@mui/material/CardContent';

export function  NameSetting(){

  return (
    <Grid display="flex" size="grow">
      <Card
        sx={{ display: 'flex' }}
        variant="outlined"
      >
        <CardContent>
          <Typography variant="h2" component="div">
            KlipperScreen setting 
          </Typography>
        </CardContent>
      </Card>
    </Grid>
  )

}