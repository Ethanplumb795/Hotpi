from flask import render_template, flash, redirect
from hello_app import app
from hello_app.forms import SetupMeasurementForm, MissionResultsForm, MissionStartForm, GetStartedForm

from SkySCPI.MeasurementControl import MeasurementControl

@app.route('/', methods=['GET', 'POST'])
@app.route('/index', methods=['GET', 'POST'])
def index():
    form = GetStartedForm()
    user = {'username': 'SkyFox User'}
    if form.validate_on_submit():
        return redirect('/setup-measurement')
    return render_template('index.html', title='Home', user=user, form=form)


@app.route('/setup-measurement', methods=['GET', 'POST'])
def setupMeasurement():
    form = SetupMeasurementForm()
    if form.validate_on_submit():
        measurement_control = MeasurementControl();
        flash('Uploading settings: Start Frequency = {}MHz, Stop Frequency = {}MHz, Integration Bandwidth = {}MHz, '
              'Resolution Bandwidth = {}, Preamp Enabled = {}, RF Atten = {}dB, Interval = {}Hz'.format(
            form.startFreq.data, form.stopFreq.data, form.intBW.data, form.resBW.data, form.preampEnabled.data,
            form.rfAtten.data, form.interval.data))
        measurement_setup_dictionary = {
            "int_bw": form.intBW.data,
            "preamp": form.preampEnabled.data,
            "rbw": form.resBW.data,
            "rf_atten": form.rfAtten.data,
            "interval": form.interval.data,
            "freq_start": form.startFreq.data,
            "freq_stop": form.stopFreq.data}
        # TODO add status to measurement_control
        measurement_control(measurement_setup_dictionary)
        measurement_control_status = True;
        if measurement_control_status:
            flash("Settings Successfully Updated!")
            return redirect('/mission-start')
        else:
            flash("Error uploading settings; check the USB connection to the FieldFox")
            return redirect('/setup-measurement')
    return render_template('measurement-setup.html', title='Setup Measurement', form=form)


@app.route('/mission-start', methods=['GET', 'POST'])
def missionStart():
    form = MissionStartForm()
    if form.validate_on_submit():
        flash("Mission initiated")
        return redirect('/mission-results')
    return render_template('mission-start.html', title='Begin Data Collection', form=form)


@app.route('/mission-results', methods=['GET', 'POST'])
def missionResults():
    form = MissionResultsForm()
    if form.validate_on_submit():
        flash("Data collection complete, retrieving data...")
        flash("Data successfully retrieved")
        image1path = '/'
        image2path = '/'
        return render_template('data-display.html', title='Data Display', image1=image1path, image2=image2path)
    return render_template('mission-results.html', title='Get Mission Results', form=form)
