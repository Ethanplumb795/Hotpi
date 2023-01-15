from flask import render_template, flash, redirect
from hotflask import app
from hotflask.forms import SetupMeasurementForm, MissionResultsForm, MissionStartForm, GetStartedForm

#from SkySCPI.MeasurementControl import MeasurementControl

@app.route('/', methods=['GET', 'POST'])
@app.route('/index', methods=['GET', 'POST'])
def index():
    form = GetStartedForm()
    user = {'username': 'Hotpi User'}
    if form.validate_on_submit():
        return redirect('/setup-measurement')
    return render_template('index.html', title='Home', user=user, form=form)


@app.route('/setup-measurement', methods=['GET', 'POST'])
def setupMeasurement():
    form = SetupMeasurementForm()
    if form.validate_on_submit():
        #measurement_control = MeasurementControl();
        flash('Uploading settings: Frequency = {}Hz, Duration = {}s, # Averaged Measurements = {}, Measurement Name = {}'.format(
            form.measFreq.data, form.duration.data, form.numAvgs.data,
            form.measName.data))
        measurement_setup_dictionary = {
            "meas_freq": form.measFreq.data,
            "duration": form.duration.data,
            "num_avgs": form.numAvgs.data,
            "meas_name": form.measName.data,
        }

        # Write to a file so Rust can execute the measurement
        f = open("measurements/" + measurement_setup_dictionary["meas_name"] + ".csv", "w")
        f.write(str(measurement_setup_dictionary["meas_freq"]) + ","
                + str(measurement_setup_dictionary["num_avgs"]) + ","
                + str(measurement_setup_dictionary["duration"]) )
        f.close()

        #measurement_control(measurement_setup_dictionary)
        #measurement_control_status = True;
        #if measurement_control_status:
        #    flash("Settings Successfully Updated!")
        #    return redirect('/mission-start')
        #else:
        #    flash("Error uploading settings; check the USB connection to the FieldFox")
        #    return redirect('/setup-measurement')
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
