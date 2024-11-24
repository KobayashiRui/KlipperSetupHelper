import Grid from '@mui/material/Grid2';
import Card from '@mui/material/Card'
import { Typography } from '@mui/material';

export default function KlipperScreenSetting(){

  return (
    <Grid size={{ xs: 12, sm:6, md: 4, lg:3 }}>
      <Card
        sx={{
          border: '2px solid',
          borderColor: 'grey.300', // 初期色
          borderRadius: '8px',
        }}
      >
        <Typography variant="h5">Klipper Screen Setting</Typography>

      </Card>
    </Grid>
  )
}